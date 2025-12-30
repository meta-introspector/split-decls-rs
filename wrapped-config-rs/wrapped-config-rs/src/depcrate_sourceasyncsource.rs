// Generated macro for AsyncSource (trait)
macro_rules! Depcrate_sourceAsyncSource {
() => {
// Module: crate::source
// Provides: {"AsyncSource"}
// Dependencies: {}
# [doc = " Describes a generic _source_ of configuration properties capable of using an async runtime."] # [doc = ""] # [doc = " At the moment this library does not implement it, although it allows using its implementations"] # [doc = " within builders.  Due to the scattered landscape of asynchronous runtimes, it is impossible to"] # [doc = " cater to all needs with one implementation.  Also, this trait might be most useful with remote"] # [doc = " configuration sources, reachable via the network, probably using HTTP protocol.  Numerous HTTP"] # [doc = " libraries exist, making it even harder to find one implementation that rules them all."] # [doc = ""] # [doc = " For those reasons, it is left to other crates to implement runtime-specific or proprietary"] # [doc = " details."] # [doc = ""] # [doc = " It is advised to use `async_trait` crate while implementing this trait."] # [doc = ""] # [doc = " See examples for sample implementation."] # [cfg (feature = "async")] # [async_trait] pub trait AsyncSource : Debug + Sync { # [doc = " Collects all configuration properties available from this source and return"] # [doc = " a Map as an async operations."] async fn collect (& self) -> Result < Map < String , Value > > ; # [doc = " Collects all configuration properties to a provided cache."] async fn collect_to (& self , cache : & mut Value) -> Result < () > { self . collect () . await ? . into_iter () . for_each (| (key , val) | set_value (cache , key , val)) ; Ok (()) } }
};
}
