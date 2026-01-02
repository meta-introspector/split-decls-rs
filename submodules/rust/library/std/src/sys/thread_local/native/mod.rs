mkuse!{use crate :: cell :: Cell ;}
mkuse!{use crate :: ptr ;}
mkmod!{eager, { 
                getname!(eager);
                getsrc!(eager);
                getpath!(eager);
                get_deps!(eager);
                get_crates!(eager);
                mkinclude!(eager);
                 
            }}
mkmod!{lazy, { 
                getname!(lazy);
                getsrc!(lazy);
                getpath!(lazy);
                get_deps!(lazy);
                get_crates!(lazy);
                mkinclude!(lazy);
                 
            }}
mkuse!{pub use eager :: Storage as EagerStorage ;}
mkuse!{pub use lazy :: Storage as LazyStorage ;}
mkitem!{# [doc (hidden)] # [allow_internal_unstable (thread_local_internals , cfg_target_thread_local , thread_local , never_type)] # [allow_internal_unsafe] # [unstable (feature = "thread_local_internals" , issue = "none")] # [rustc_macro_transparency = "semitransparent"] pub macro thread_local_inner { (@ key $ t : ty , const $ init : expr) => { { const __INIT : $ t = $ init ; unsafe { $ crate :: thread :: LocalKey :: new (const { if $ crate :: mem :: needs_drop ::<$ t > () { | _ | { # [thread_local] static VAL : $ crate :: thread :: local_impl :: EagerStorage <$ t > = $ crate :: thread :: local_impl :: EagerStorage :: new (__INIT) ; VAL . get () } } else { | _ | { # [thread_local] static VAL : $ t = __INIT ; & VAL } } }) } } } , (@ key $ t : ty , $ init : expr) => { { # [inline] fn __init () -> $ t { $ init } unsafe { $ crate :: thread :: LocalKey :: new (const { if $ crate :: mem :: needs_drop ::<$ t > () { | init | { # [thread_local] static VAL : $ crate :: thread :: local_impl :: LazyStorage <$ t , () > = $ crate :: thread :: local_impl :: LazyStorage :: new () ; VAL . get_or_init (init , __init) } } else { | init | { # [thread_local] static VAL : $ crate :: thread :: local_impl :: LazyStorage <$ t , !> = $ crate :: thread :: local_impl :: LazyStorage :: new () ; VAL . get_or_init (init , __init) } } }) } } } , ($ (# [$ attr : meta]) * $ vis : vis $ name : ident , $ t : ty , $ ($ init : tt) *) => { $ (# [$ attr]) * $ vis const $ name : $ crate :: thread :: LocalKey <$ t > = $ crate :: thread :: local_impl :: thread_local_inner ! (@ key $ t , $ ($ init) *) ; } , }}
mkitem!{# [rustc_macro_transparency = "semitransparent"] pub (crate) macro local_pointer { () => { } , ($ vis : vis static $ name : ident ; $ ($ rest : tt) *) => { # [thread_local] $ vis static $ name : $ crate :: sys :: thread_local :: LocalPointer = $ crate :: sys :: thread_local :: LocalPointer :: __new () ; $ crate :: sys :: thread_local :: local_pointer ! { $ ($ rest) * } } , }}
mkitem!{mkstruct!{pub (crate) struct LocalPointer { p : Cell < * mut () > , }}}
mkitem!{mkimpl!{impl LocalPointer { pub const fn __new () -> LocalPointer { LocalPointer { p : Cell :: new (ptr :: null_mut ()) } } pub fn get (& self) -> * mut () { self . p . get () } pub fn set (& self , p : * mut ()) { self . p . set (p) } }}}