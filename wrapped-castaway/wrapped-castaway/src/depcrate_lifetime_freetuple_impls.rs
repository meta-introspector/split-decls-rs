// Generated macro for tuple_impls (macro)
macro_rules! Depcrate_lifetime_freetuple_impls {
() => {
// Module: crate::lifetime_free
// Provides: {"tuple_impls"}
// Dependencies: {}
macro_rules ! tuple_impls { ($ ($ ($ name : ident) +,) +) => { $ (unsafe impl <$ ($ name : LifetimeFree) ,+> LifetimeFree for ($ ($ name ,) +) { }) + } ; }
};
}
