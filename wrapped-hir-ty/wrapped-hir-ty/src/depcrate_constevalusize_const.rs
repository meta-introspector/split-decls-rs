// Generated macro for usize_const (function)
macro_rules! Depcrate_constevalusize_const {
() => {
// Module: crate::consteval
// Provides: {"usize_const"}
// Dependencies: {}
# [doc = " Interns a possibly-unknown target usize"] pub fn usize_const < 'db > (db : & 'db dyn HirDatabase , value : Option < u128 > , krate : Crate) -> Const < 'db > { intern_const_ref (db , & value . map_or (LiteralConstRef :: Unknown , LiteralConstRef :: UInt) , Ty :: new_uint (DbInterner :: new_with (db , Some (krate) , None) , rustc_type_ir :: UintTy :: Usize) , krate ,) }
};
}
