macro_rules! deps {
    () => {
        TomlTrimPaths!();
        TomlOptLevel!();
        ProfilePackageSpec!();
        TomlDebugInfo!();
        StringOrBool!();
    };
}

macro_rules! TomlProfile {
    () => {
        deps!();
        # [derive (Deserialize , Serialize , Clone , Debug , Default , Eq , PartialEq)] # [serde (default , rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlProfile { pub opt_level : Option < TomlOptLevel > , pub lto : Option < StringOrBool > , pub codegen_backend : Option < String > , pub codegen_units : Option < u32 > , pub debug : Option < TomlDebugInfo > , pub split_debuginfo : Option < String > , pub debug_assertions : Option < bool > , pub rpath : Option < bool > , pub panic : Option < String > , pub overflow_checks : Option < bool > , pub incremental : Option < bool > , pub dir_name : Option < String > , pub inherits : Option < String > , pub strip : Option < StringOrBool > , pub rustflags : Option < Vec < String > > , pub package : Option < BTreeMap < ProfilePackageSpec , TomlProfile > > , pub build_override : Option < Box < TomlProfile > > , # [doc = " Unstable feature `-Ztrim-paths`."] pub trim_paths : Option < TomlTrimPaths > , # [doc = " Unstable feature `hint-mostly-unused`"] pub hint_mostly_unused : Option < bool > , }
    };
}

TomlProfile!()