// Generated macro for __lazy_static_internal (macro)
macro_rules! Depcrate__lazy_static_internal {
() => {
// Module: crate
// Provides: {"__lazy_static_internal"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! __lazy_static_internal { ($ (# [$ attr : meta]) * ($ ($ vis : tt) *) static ref $ N : ident : $ T : ty = $ init : expr ; $ ($ t : tt) *) => { # [allow (missing_copy_implementations)] # [allow (non_camel_case_types)] # [allow (dead_code)] $ (# [$ attr]) * $ ($ vis) * struct $ N { __private_field : () } # [doc (hidden)] $ ($ vis) * static $ N : $ N = $ N { __private_field : () } ; impl :: core :: ops :: Deref for $ N { type Target = $ T ; fn deref (& self) -> &$ T { # [inline (always)] fn __static_ref_initialize () -> $ T { $ init } # [inline (always)] fn __stability () -> &'static $ T { static LAZY : $ crate :: lazy_static :: Lazy <$ T > = $ crate :: lazy_static :: Lazy { init : __static_ref_initialize , _p : core :: marker :: PhantomData , } ; LAZY . get () } __stability () } } $ crate :: lazy_static ! ($ ($ t) *) ; } ; () => () }
};
}
