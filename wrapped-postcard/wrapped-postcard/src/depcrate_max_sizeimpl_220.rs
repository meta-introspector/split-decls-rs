// Generated macro for impl_220 (impl)
macro_rules! Depcrate_max_sizeimpl_220 {
() => {
// Module: crate::max_size
// Provides: {"impl_220"}
// Dependencies: {}
# [cfg (all (feature = "core-num-saturating" , feature = "experimental-derive"))] # [cfg_attr (docsrs , doc (cfg (feature = "core-num-saturating")))] impl < T : MaxSize > MaxSize for core :: num :: Saturating < T > { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE ; }
};
}
