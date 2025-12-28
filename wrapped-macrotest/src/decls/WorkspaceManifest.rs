macro_rules! deps {
    () => {
        Workspace!();
        Patch!();
        RegistryPatch!();
    };
}

macro_rules! WorkspaceManifest {
    () => {
        deps!();
        # [derive (Deserialize , Default , Debug)] pub struct WorkspaceManifest { # [serde (default)] pub workspace : Workspace , # [serde (default)] pub patch : Map < String , RegistryPatch > , # [serde (default)] pub replace : Map < String , Patch > , }
    };
}

WorkspaceManifest!()