macro_rules! deps {
    () => {
        LazyStatic!();
    };
}

macro_rules! __lazy_static_internal {
    () => {
        deps!();
        # [macro_export (local_inner_macros)] # [doc (hidden)] macro_rules ! __lazy_static_internal { ($ (# [$ attr : meta]) * ($ ($ vis : tt) *) static ref $ N : ident : $ T : ty = $ e : expr ; $ ($ t : tt) *) => { __lazy_static_internal ! (@ MAKE TY , $ (# [$ attr]) *, ($ ($ vis) *) , $ N) ; __lazy_static_internal ! (@ TAIL , $ N : $ T = $ e) ; lazy_static ! ($ ($ t) *) ; } ; (@ TAIL , $ N : ident : $ T : ty = $ e : expr) => { impl $ crate :: __Deref for $ N { type Target = $ T ; fn deref (& self) -> &$ T { # [inline (always)] fn __static_ref_initialize () -> $ T { $ e } # [inline (always)] fn __stability () -> &'static $ T { __lazy_static_create ! (LAZY , $ T) ; LAZY . get (__static_ref_initialize) } __stability () } } impl $ crate :: LazyStatic for $ N { fn initialize (lazy : & Self) { let _ = &** lazy ; } } } ; (@ MAKE TY , $ (# [$ attr : meta]) *, ($ ($ vis : tt) *) , $ N : ident) => { # [allow (missing_copy_implementations)] # [allow (non_camel_case_types)] # [allow (dead_code)] $ (# [$ attr]) * $ ($ vis) * struct $ N { __private_field : () } # [doc (hidden)] # [allow (non_upper_case_globals)] $ ($ vis) * static $ N : $ N = $ N { __private_field : () } ; } ; () => () }
    };
}

__lazy_static_internal!();