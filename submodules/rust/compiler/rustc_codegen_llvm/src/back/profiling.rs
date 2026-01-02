mkuse!{use std :: ffi :: { CStr , c_void } ;}
mkuse!{use std :: os :: raw :: c_char ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use measureme :: event_id :: SEPARATOR_BYTE ;}
mkuse!{use measureme :: { EventId , StringComponent , StringId } ;}
mkuse!{use rustc_data_structures :: profiling :: { SelfProfiler , TimingGuard } ;}

macro_rules! llvm_args_to_string_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function llvm_args_to_string_id in module {}", module_path!());
    };
}

mkfn!{
    llvm_args_to_string_id_introspect!();
    fn llvm_args_to_string_id (profiler : & SelfProfiler , pass_name : & str , ir_name : & str) -> EventId { let pass_name = profiler . get_or_alloc_cached_string (pass_name) ; let mut components = vec ! [StringComponent :: Ref (pass_name)] ; let parentheses : & [_] = & ['(' , ')'] ; let trimmed = ir_name . trim_matches (parentheses) ; for part in trimmed . split (", ") { let demangled_ir_name = rustc_demangle :: demangle (part) . to_string () ; let ir_name = profiler . get_or_alloc_cached_string (demangled_ir_name) ; components . push (StringComponent :: Value (SEPARATOR_BYTE)) ; components . push (StringComponent :: Ref (ir_name)) ; } EventId :: from_label (profiler . alloc_string (components . as_slice ())) }
}
mkitem!{mkstruct!{pub (crate) struct LlvmSelfProfiler < 'a > { profiler : Arc < SelfProfiler > , stack : Vec < TimingGuard < 'a > > , llvm_pass_event_kind : StringId , }}}
mkitem!{mkimpl!{impl < 'a > LlvmSelfProfiler < 'a > { pub (crate) fn new (profiler : Arc < SelfProfiler >) -> Self { let llvm_pass_event_kind = profiler . alloc_string ("LLVM Pass") ; Self { profiler , stack : Vec :: default () , llvm_pass_event_kind } } fn before_pass_callback (& 'a mut self , pass_name : & str , ir_name : & str) { let event_id = llvm_args_to_string_id (& self . profiler , pass_name , ir_name) ; self . stack . push (TimingGuard :: start (& self . profiler , self . llvm_pass_event_kind , event_id)) ; } fn after_pass_callback (& mut self) { self . stack . pop () ; } }}}

macro_rules! selfprofile_before_pass_callback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function selfprofile_before_pass_callback in module {}", module_path!());
    };
}

mkfn!{
    selfprofile_before_pass_callback_introspect!();
    pub (crate) unsafe extern "C" fn selfprofile_before_pass_callback (llvm_self_profiler : * mut c_void , pass_name : * const c_char , ir_name : * const c_char ,) { unsafe { let llvm_self_profiler = & mut * (llvm_self_profiler as * mut LlvmSelfProfiler < '_ >) ; let pass_name = CStr :: from_ptr (pass_name) . to_str () . expect ("valid UTF-8") ; let ir_name = CStr :: from_ptr (ir_name) . to_str () . expect ("valid UTF-8") ; llvm_self_profiler . before_pass_callback (pass_name , ir_name) ; } }
}

macro_rules! selfprofile_after_pass_callback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function selfprofile_after_pass_callback in module {}", module_path!());
    };
}

mkfn!{
    selfprofile_after_pass_callback_introspect!();
    pub (crate) unsafe extern "C" fn selfprofile_after_pass_callback (llvm_self_profiler : * mut c_void) { let llvm_self_profiler = unsafe { & mut * (llvm_self_profiler as * mut LlvmSelfProfiler < '_ >) } ; llvm_self_profiler . after_pass_callback () ; }
}