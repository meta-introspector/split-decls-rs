// Generated macro for impl_41 (impl)
macro_rules! Depcrate_item_implimpl_41 {
() => {
// Module: crate::item_impl
// Provides: {"impl_41"}
// Dependencies: {}
impl Display for Op { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . op) ? ; if self . form == OpForm :: Assign { write ! (f , "Assign") ? ; } Ok (()) } }
};
}
