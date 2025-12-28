macro_rules! deps {
    () => {
        CargoMetadataConfig!();
        ProjectJson!();
    };
}

macro_rules! RustSourceWorkspaceConfig {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub enum RustSourceWorkspaceConfig { CargoMetadata (CargoMetadataConfig) , Json (ProjectJson) , }
    };
}

RustSourceWorkspaceConfig!()