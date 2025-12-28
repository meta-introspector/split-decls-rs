macro_rules! deps {
    () => {
        Patch!();
    };
}

macro_rules! RegistryPatch {
    () => {
        deps!();
        # [derive (Serialize , Deserialize , Clone , Debug)] # [serde (transparent)] pub struct RegistryPatch { crates : Map < String , Patch > , }
    };
}

RegistryPatch!()