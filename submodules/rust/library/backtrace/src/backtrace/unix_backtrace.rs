mkuse!{use split_decls_genesis :: ourprelude :: * ;}
mkuse!{use std :: mem ;}
mkuse!{use libc :: { c_void , c_int } ;}
mkuse!{use backtrace :: { Callback , Context } ;}
mkitem!{mkstruct!{struct BacktraceContext { addr : * mut c_void , }}}
mkitem!{mkimpl!{impl Context for BacktraceContext { fn ip (& self) -> * mut c_void { self . addr } fn symbol_address (& self) -> * mut c_void { self . addr } }}}
mkitem!{extern { fn backtrace (buf : * mut * mut c_void , sz : c_int) -> c_int ; }}

macro_rules! trace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trace in module {}", module_path!());
    };
}

mkfn!{
    trace_introspect!();
    #[inline (never)] pub fn trace (mut cb : & mut Callback) { const SIZE : usize = 100 ; let mut buf : [* mut c_void ; SIZE] ; let cnt ; unsafe { buf = mem :: zeroed () ; cnt = backtrace (buf . as_mut_ptr () , SIZE as c_int) ; } for addr in buf [.. cnt as usize] . iter () { let cx = BacktraceContext { addr : * addr } ; if ! cb (& cx) { return } } }
}