// Generated macro for impl_from (macro)
macro_rules! Depcrateimpl_from {
() => {
// Module: crate
// Provides: {"impl_from"}
// Dependencies: {}
macro_rules ! impl_from { ($ ($ err_ty : ty => $ variant : ident) ,* $ (,) *) => { $ (impl From <$ err_ty > for Error { fn from (e : $ err_ty) -> Error { Error ::$ variant (e) } }) * } }
};
}
