// Generated macro for Source (trait)
macro_rules! Depcrate_sourceSource {
() => {
// Module: crate::source
// Provides: {"Source"}
// Dependencies: {}
# [doc = " Describes a generic _source_ of configuration properties."] pub trait Source : Debug { fn clone_into_box (& self) -> Box < dyn Source + Send + Sync > ; # [doc = " Collect all configuration properties available from this source into"] # [doc = " a [`Map`]."] fn collect (& self) -> Result < Map < String , Value > > ; # [doc = " Collects all configuration properties to a provided cache."] fn collect_to (& self , cache : & mut Value) -> Result < () > { self . collect () ? . into_iter () . for_each (| (key , val) | set_value (cache , key , val)) ; Ok (()) } }
};
}
