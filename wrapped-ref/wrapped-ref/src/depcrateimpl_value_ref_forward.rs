// Generated macro for impl_value_ref_forward (macro)
macro_rules! Depcrateimpl_value_ref_forward {
() => {
// Module: crate
// Provides: {"impl_value_ref_forward"}
// Dependencies: {}
macro_rules ! impl_value_ref_forward { ({ $ ($ r : tt) * } => $ bind : ident => { $ ($ forward : tt) * }) => { $ ($ r) * { fn stream_ref < S : Stream <'sval > + ? Sized > (& self , stream : & mut S) -> Result { let $ bind = self ; ($ ($ forward) *) . stream_ref (stream) } } } ; }
};
}
