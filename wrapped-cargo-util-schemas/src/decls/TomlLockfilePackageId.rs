macro_rules! deps {
    () => {
        TomlLockfileSourceId!();
    };
}

macro_rules! TomlLockfilePackageId {
    () => {
        deps!();
        # [doc = " Serialization of package IDs."] # [doc = ""] # [doc = " The version and source are only included when necessary to disambiguate between packages:"] # [doc = " - If multiple packages share the same name, the version is included."] # [doc = " - If multiple packages share the same name and version, the source is included."] # [derive (Debug , PartialOrd , Ord , PartialEq , Eq , Hash , Clone)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlLockfilePackageId { pub name : String , pub version : Option < String > , pub source : Option < TomlLockfileSourceId > , }
    };
}

TomlLockfilePackageId!()