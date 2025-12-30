// Generated macro for impl_insert (macro)
macro_rules! Depcrate_transliterate_compile_pass2impl_insert {
() => {
// Module: crate::transliterate::compile::pass2
// Provides: {"impl_insert"}
// Dependencies: {}
macro_rules ! impl_insert { ($ fn_name : ident , $ field : ident , $ elt_type : ty , $ ($ next_field : tt) *) => { fn $ fn_name (& mut self , elt : $ elt_type) -> char { debug_assert ! (self .$ field . current < self .$ ($ next_field) *) ; let standin = char :: try_from (self .$ field . current) . unwrap () ; self .$ field . vec . push (elt) ; self .$ field . current += 1 ; standin } } ; }
};
}
