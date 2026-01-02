mkuse!{use super :: super :: windows_sys :: * ;}
mkuse!{use core :: ffi :: c_void ;}
mkitem!{mkstruct!{# [derive (Clone , Copy)] pub struct Frame { base_address : * mut c_void , ip : * mut c_void , sp : * mut c_void , # [cfg (not (target_env = "gnu"))] inline_context : Option < u32 > , }}}
mkitem!{mkimpl!{unsafe impl Send for Frame { }}}
mkitem!{mkimpl!{unsafe impl Sync for Frame { }}}
mkitem!{mkimpl!{impl Frame { pub fn ip (& self) -> * mut c_void { self . ip } pub fn sp (& self) -> * mut c_void { self . sp } pub fn symbol_address (& self) -> * mut c_void { self . ip } pub fn module_base_address (& self) -> Option < * mut c_void > { Some (self . base_address) } # [cfg (not (target_env = "gnu"))] pub fn inline_context (& self) -> Option < u32 > { self . inline_context } }}}
mkitem!{mkstruct!{# [repr (C , align (16))] struct MyContext (CONTEXT) ;}}
mkitem!{mkimpl!{# [cfg (any (target_arch = "x86_64" , target_arch = "arm64ec"))] impl MyContext { # [inline (always)] fn ip (& self) -> u64 { self . 0 . Rip } # [inline (always)] fn sp (& self) -> u64 { self . 0 . Rsp } }}}
mkitem!{mkimpl!{# [cfg (target_arch = "aarch64")] impl MyContext { # [inline (always)] fn ip (& self) -> usize { self . 0 . Pc as usize } # [inline (always)] fn sp (& self) -> usize { self . 0 . Sp as usize } }}}

macro_rules! trace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trace in module {}", module_path!());
    };
}

mkfn!{
    trace_introspect!();
    # [inline (always)] pub unsafe fn trace (cb : & mut dyn FnMut (& super :: Frame) -> bool) { use core :: ptr ; let mut context = unsafe { core :: mem :: zeroed :: < MyContext > () } ; unsafe { RtlCaptureContext (& mut context . 0) } ; loop { let ip = context . ip () ; let mut base = 0 ; let fn_entry = unsafe { RtlLookupFunctionEntry (ip , & mut base , ptr :: null_mut ()) } ; if fn_entry . is_null () { break ; } let frame = super :: Frame { inner : Frame { base_address : base as * mut c_void , ip : ip as * mut c_void , sp : context . sp () as * mut c_void , # [cfg (not (target_env = "gnu"))] inline_context : None , } , } ; if ! cb (& frame) { break ; } let previous_ip = ip ; let previous_sp = context . sp () ; let mut handler_data = 0usize ; let mut establisher_frame = 0 ; unsafe { RtlVirtualUnwind (0 , base , ip , fn_entry , & mut context . 0 , ptr :: addr_of_mut ! (handler_data) . cast :: < * mut c_void > () , & mut establisher_frame , ptr :: null_mut () ,) ; } let ip = context . ip () ; if ip == 0 || (ip == previous_ip && context . sp () == previous_sp) { break ; } } }
}