mkuse!{use super :: super :: { dbghelp , windows_sys :: * } ;}
mkuse!{use core :: ffi :: c_void ;}
mkuse!{use core :: mem ;}
mkitem!{mkenum!{# [derive (Clone , Copy)] pub enum StackFrame { New (STACKFRAME_EX) , Old (STACKFRAME64) , }}}
mkitem!{mkstruct!{# [derive (Clone , Copy)] pub struct Frame { pub (crate) stack_frame : StackFrame , base_address : * mut c_void , }}}
mkitem!{mkimpl!{unsafe impl Send for Frame { }}}
mkitem!{mkimpl!{unsafe impl Sync for Frame { }}}
mkitem!{mkimpl!{impl Frame { pub fn ip (& self) -> * mut c_void { self . addr_pc () . Offset as * mut _ } pub fn sp (& self) -> * mut c_void { self . addr_stack () . Offset as * mut _ } pub fn symbol_address (& self) -> * mut c_void { self . ip () } pub fn module_base_address (& self) -> Option < * mut c_void > { Some (self . base_address) } # [cfg (not (target_env = "gnu"))] pub fn inline_context (& self) -> Option < u32 > { match self . stack_frame { StackFrame :: New (ref new) => Some (new . InlineFrameContext) , StackFrame :: Old (_) => None , } } fn addr_pc (& self) -> & ADDRESS64 { match self . stack_frame { StackFrame :: New (ref new) => & new . AddrPC , StackFrame :: Old (ref old) => & old . AddrPC , } } fn addr_pc_mut (& mut self) -> & mut ADDRESS64 { match self . stack_frame { StackFrame :: New (ref mut new) => & mut new . AddrPC , StackFrame :: Old (ref mut old) => & mut old . AddrPC , } } fn addr_frame_mut (& mut self) -> & mut ADDRESS64 { match self . stack_frame { StackFrame :: New (ref mut new) => & mut new . AddrFrame , StackFrame :: Old (ref mut old) => & mut old . AddrFrame , } } fn addr_stack (& self) -> & ADDRESS64 { match self . stack_frame { StackFrame :: New (ref new) => & new . AddrStack , StackFrame :: Old (ref old) => & old . AddrStack , } } fn addr_stack_mut (& mut self) -> & mut ADDRESS64 { match self . stack_frame { StackFrame :: New (ref mut new) => & mut new . AddrStack , StackFrame :: Old (ref mut old) => & mut old . AddrStack , } } }}}
mkitem!{mkstruct!{# [repr (C , align (16))] struct MyContext (CONTEXT) ;}}

macro_rules! trace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trace in module {}", module_path!());
    };
}

mkfn!{
    trace_introspect!();
    # [inline (always)] pub unsafe fn trace (cb : & mut dyn FnMut (& super :: Frame) -> bool) { let process = unsafe { GetCurrentProcess () } ; let thread = unsafe { GetCurrentThread () } ; let mut context = unsafe { mem :: zeroed :: < MyContext > () } ; unsafe { RtlCaptureContext (& mut context . 0) } ; let dbghelp = match dbghelp :: init () { Ok (dbghelp) => dbghelp , Err (()) => return , } ; let function_table_access = dbghelp . SymFunctionTableAccess64 () ; let get_module_base = dbghelp . SymGetModuleBase64 () ; match unsafe { (* dbghelp . dbghelp ()) . StackWalkEx () } { # [allow (non_snake_case)] Some (StackWalkEx) => { let mut inner : STACKFRAME_EX = unsafe { mem :: zeroed () } ; inner . StackFrameSize = mem :: size_of :: < STACKFRAME_EX > () as u32 ; let mut frame = super :: Frame { inner : Frame { stack_frame : StackFrame :: New (inner) , base_address : 0 as _ , } , } ; let image = init_frame (& mut frame . inner , & context . 0) ; let frame_ptr = match & mut frame . inner . stack_frame { StackFrame :: New (ptr) => ptr as * mut STACKFRAME_EX , _ => unreachable ! () , } ; while unsafe { StackWalkEx (image as u32 , process , thread , frame_ptr , & mut context . 0 as * mut CONTEXT as * mut _ , None , Some (function_table_access) , Some (get_module_base) , None , 0 ,) == TRUE } { frame . inner . base_address = unsafe { get_module_base (process , frame . ip () as _) as _ } ; if ! cb (& frame) { break ; } } } None => { let mut frame = super :: Frame { inner : Frame { stack_frame : StackFrame :: Old (unsafe { mem :: zeroed () }) , base_address : 0 as _ , } , } ; let image = init_frame (& mut frame . inner , & context . 0) ; let frame_ptr = match & mut frame . inner . stack_frame { StackFrame :: Old (ptr) => ptr as * mut STACKFRAME64 , _ => unreachable ! () , } ; while unsafe { dbghelp . StackWalk64 () (image as u32 , process , thread , frame_ptr , & mut context . 0 as * mut CONTEXT as * mut _ , None , Some (function_table_access) , Some (get_module_base) , None ,) == TRUE } { frame . inner . base_address = unsafe { get_module_base (process , frame . ip () as _) as _ } ; if ! cb (& frame) { break ; } } } } }
}

macro_rules! init_frame_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init_frame in module {}", module_path!());
    };
}

mkfn!{
    init_frame_introspect!();
    # [cfg (target_arch = "x86")] fn init_frame (frame : & mut Frame , ctx : & CONTEXT) -> u16 { frame . addr_pc_mut () . Offset = ctx . Eip as u64 ; frame . addr_pc_mut () . Mode = AddrModeFlat ; frame . addr_stack_mut () . Offset = ctx . Esp as u64 ; frame . addr_stack_mut () . Mode = AddrModeFlat ; frame . addr_frame_mut () . Offset = ctx . Ebp as u64 ; frame . addr_frame_mut () . Mode = AddrModeFlat ; IMAGE_FILE_MACHINE_I386 }
}

macro_rules! init_frame_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init_frame in module {}", module_path!());
    };
}

mkfn!{
    init_frame_introspect!();
    # [cfg (target_arch = "arm")] fn init_frame (frame : & mut Frame , ctx : & CONTEXT) -> u16 { frame . addr_pc_mut () . Offset = ctx . Pc as u64 ; frame . addr_pc_mut () . Mode = AddrModeFlat ; frame . addr_stack_mut () . Offset = ctx . Sp as u64 ; frame . addr_stack_mut () . Mode = AddrModeFlat ; unsafe { frame . addr_frame_mut () . Offset = ctx . R11 as u64 ; } frame . addr_frame_mut () . Mode = AddrModeFlat ; IMAGE_FILE_MACHINE_ARMNT }
}