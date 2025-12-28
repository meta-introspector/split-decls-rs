macro_rules! deps {
    () => {
        TomlProfile!();
    };
}

macro_rules! TomlProfiles {
    () => {
        deps!();
        # [derive (Deserialize , Serialize , Clone , Debug , Default)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlProfiles (pub BTreeMap < ProfileName , TomlProfile >) ;
    };
}

TomlProfiles!()