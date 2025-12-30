// Generated macro for impl_829 (impl)
macro_rules! Depcrate_strategy_flattenimpl_829 {
() => {
// Module: crate::strategy::flatten
// Provides: {"impl_829"}
// Dependencies: {}
impl < S : ValueTree > fmt :: Debug for FlattenValueTree < S > where S :: Value : Strategy , S : fmt :: Debug , < S :: Value as Strategy > :: Tree : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("FlattenValueTree") . field ("meta" , & self . meta) . field ("current" , & self . current) . field ("final_complication" , & self . final_complication) . field ("complicate_regen_remaining" , & self . complicate_regen_remaining ,) . finish () } }
};
}
