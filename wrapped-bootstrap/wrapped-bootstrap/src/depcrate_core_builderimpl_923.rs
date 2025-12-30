// Generated macro for impl_923 (impl)
macro_rules! Depcrate_core_builderimpl_923 {
() => {
// Module: crate::core::builder
// Provides: {"impl_923"}
// Dependencies: {}
impl Debug for TaskPath { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if let Some (kind) = & self . kind { write ! (f , "{}::" , kind . as_str ()) ? ; } write ! (f , "{}" , self . path . display ()) } }
};
}
