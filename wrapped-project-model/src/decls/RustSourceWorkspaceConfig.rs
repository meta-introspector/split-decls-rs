macro_rules! RustSourceWorkspaceConfig {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub enum RustSourceWorkspaceConfig { CargoMetadata (CargoMetadataConfig) , Json (ProjectJson) , }
    };
}

RustSourceWorkspaceConfig!()