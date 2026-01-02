mkuse!{use alloc :: boxed :: Box ;}
mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use core :: ffi :: c_void ;}
mkitem!{unsafe extern "Rust" { fn miri_backtrace_size (flags : u64) -> usize ; fn miri_get_backtrace (flags : u64 , buf : * mut * mut ()) ; fn miri_resolve_frame (ptr : * mut () , flags : u64) -> MiriFrame ; fn miri_resolve_frame_names (ptr : * mut () , flags : u64 , name_buf : * mut u8 , filename_buf : * mut u8) ; }}
mkitem!{mkstruct!{# [repr (C)] pub struct MiriFrame { pub name_len : usize , pub filename_len : usize , pub lineno : u32 , pub colno : u32 , pub fn_ptr : * mut c_void , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug)] pub struct FullMiriFrame { pub name : Box < [u8] > , pub filename : Box < [u8] > , pub lineno : u32 , pub colno : u32 , pub fn_ptr : * mut c_void , }}}
mkitem!{mkstruct!{# [derive (Debug , Clone)] pub struct Frame { pub addr : * mut c_void , pub inner : FullMiriFrame , }}}
mkitem!{mkimpl!{unsafe impl Send for Frame { }}}
mkitem!{mkimpl!{unsafe impl Sync for Frame { }}}
mkitem!{mkimpl!{impl Frame { pub fn ip (& self) -> * mut c_void { self . addr } pub fn sp (& self) -> * mut c_void { core :: ptr :: null_mut () } pub fn symbol_address (& self) -> * mut c_void { self . inner . fn_ptr } pub fn module_base_address (& self) -> Option < * mut c_void > { None } }}}

macro_rules! trace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trace in module {}", module_path!());
    };
}

mkfn!{
    trace_introspect!();
    pub unsafe fn trace < F : FnMut (& super :: Frame) -> bool > (cb : F) { unsafe { trace_unsynchronized (cb) } ; }
}

macro_rules! resolve_addr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve_addr in module {}", module_path!());
    };
}

mkfn!{
    resolve_addr_introspect!();
    pub fn resolve_addr (ptr : * mut c_void) -> Frame { let frame = unsafe { miri_resolve_frame (ptr . cast :: < () > () , 1) } ; let mut name = Vec :: with_capacity (frame . name_len) ; let mut filename = Vec :: with_capacity (frame . filename_len) ; unsafe { miri_resolve_frame_names (ptr . cast :: < () > () , 0 , name . as_mut_ptr () , filename . as_mut_ptr () ,) ; name . set_len (frame . name_len) ; filename . set_len (frame . filename_len) ; } Frame { addr : ptr , inner : FullMiriFrame { name : name . into () , filename : filename . into () , lineno : frame . lineno , colno : frame . colno , fn_ptr : frame . fn_ptr , } , } }
}

macro_rules! trace_unsynchronized_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trace_unsynchronized in module {}", module_path!());
    };
}

mkfn!{
    trace_unsynchronized_introspect!();
    unsafe fn trace_unsynchronized < F : FnMut (& super :: Frame) -> bool > (mut cb : F) { let len = unsafe { miri_backtrace_size (0) } ; let mut frames = Vec :: with_capacity (len) ; unsafe { miri_get_backtrace (1 , frames . as_mut_ptr ()) ; frames . set_len (len) ; } for ptr in frames . iter () { let frame = resolve_addr ((* ptr) . cast :: < c_void > ()) ; if ! cb (& super :: Frame { inner : frame }) { return ; } } }
}