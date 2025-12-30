// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl ServerCacheWithResumptionDelay { fn new (delay : u32) -> Arc < Self > { Arc :: new (Self { delay , storage : server :: ServerSessionMemoryCache :: new (32) , }) } }
};
}
