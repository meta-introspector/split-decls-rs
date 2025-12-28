macro_rules! TomlDebugInfo {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum TomlDebugInfo { None , LineDirectivesOnly , LineTablesOnly , Limited , Full , }
    };
}

TomlDebugInfo!();