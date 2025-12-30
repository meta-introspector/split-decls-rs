// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl ClientCacheWithoutKxHints { fn new (delay : u32) -> Arc < Self > { Arc :: new (Self { delay , storage : Arc :: new (client :: ClientSessionMemoryCache :: new (32)) , }) } }
};
}
