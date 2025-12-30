// Generated macro for impl_1096 (impl)
macro_rules! Depcrate_tz_tzifimpl_1096 {
() => {
// Module: crate::tz::tzif
// Provides: {"impl_1096"}
// Dependencies: {}
impl core :: fmt :: Display for shared :: TzifIndicator { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { match * self { shared :: TzifIndicator :: LocalWall => write ! (f , "local/wall") , shared :: TzifIndicator :: LocalStandard => write ! (f , "local/std") , shared :: TzifIndicator :: UTStandard => write ! (f , "ut/std") , } } }
};
}
