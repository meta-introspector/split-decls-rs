// Generated macro for impl_1014 (impl)
macro_rules! Depcrate_strategy_unionsimpl_1014 {
() => {
// Module: crate::strategy::unions
// Provides: {"impl_1014"}
// Dependencies: {}
impl < T : Strategy > fmt :: Debug for UnionValueTree < T > where T :: Tree : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("UnionValueTree") . field ("options" , & self . options) . field ("pick" , & self . pick) . field ("min_pick" , & self . min_pick) . field ("prev_pick" , & self . prev_pick) . finish () } }
};
}
