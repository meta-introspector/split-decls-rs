// Generated macro for impl_114 (impl)
macro_rules! Depcrate_typesimpl_114 {
() => {
// Module: crate::types
// Provides: {"impl_114"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for SourceItemOrderingWithinModuleItemGroupings { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let description = "The available options for configuring an ordering within module item groups are: \
                    \"all\", \"none\", or a list of module item group names \
                    (as configured with the `module-item-order-groupings` configuration option)." ; match StringOrVecOfString :: deserialize (deserializer) { Ok (StringOrVecOfString :: String (preset)) if preset == "all" => { Ok (SourceItemOrderingWithinModuleItemGroupings :: All) } , Ok (StringOrVecOfString :: String (preset)) if preset == "none" => { Ok (SourceItemOrderingWithinModuleItemGroupings :: None) } , Ok (StringOrVecOfString :: String (preset)) => Err (de :: Error :: custom (format ! ("Unknown configuration option: {preset}.\n{description}"))) , Ok (StringOrVecOfString :: Vec (groupings)) => { Ok (SourceItemOrderingWithinModuleItemGroupings :: Custom (groupings)) } , Err (e) => Err (de :: Error :: custom (format ! ("{e}\n{description}"))) , } } }
};
}
