// Generated macro for impl_format (macro)
macro_rules! Depcrate_iter_formatimpl_format {
() => {
// Module: crate::iter_format
// Provides: {"impl_format"}
// Dependencies: {}
macro_rules ! impl_format { ($ ($ fmt_trait : ident) *) => { $ (impl <'a , I > fmt ::$ fmt_trait for Format <'a , I > where I : Iterator , I :: Item : fmt ::$ fmt_trait , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . format (f , fmt ::$ fmt_trait :: fmt) } }) * } }
};
}
