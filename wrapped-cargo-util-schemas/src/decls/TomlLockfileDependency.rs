macro_rules! deps {
    () => {
        TomlLockfilePackageId!();
        TomlLockfileMetadata!();
        TomlLockfileSourceId!();
    };
}

macro_rules! TomlLockfileDependency {
    () => {
        deps!();
        # [doc = " Serialization of lockfiles dependencies"] # [derive (Serialize , Deserialize , Debug , PartialOrd , Ord , PartialEq , Eq)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlLockfileDependency { # [doc = " The name of the dependency."] pub name : String , # [doc = " The version of the dependency."] pub version : String , # [doc = " The source of the dependency."] # [doc = ""] # [doc = " Cargo does not serialize path dependencies."] pub source : Option < TomlLockfileSourceId > , # [doc = " The checksum of the dependency."] # [doc = ""] # [doc = " In older lockfiles, checksums were not stored here and instead on a separate `[metadata]`"] # [doc = " table (see [`TomlLockfileMetadata`])."] pub checksum : Option < String > , # [doc = " The transitive dependencies used by this dependency."] pub dependencies : Option < Vec < TomlLockfilePackageId > > , # [doc = " The replace of the dependency."] pub replace : Option < TomlLockfilePackageId > , }
    };
}

TomlLockfileDependency!();