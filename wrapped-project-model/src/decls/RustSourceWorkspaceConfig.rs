macro_rules! deps {
    () => {
        ProjectJson!();
        CargoMetadataConfig!();
    };
}

macro_rules! RustSourceWorkspaceConfig {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub enum RustSourceWorkspaceConfig { CargoMetadata (CargoMetadataConfig) , Json (ProjectJson) , }
    };
}

RustSourceWorkspaceConfig!();