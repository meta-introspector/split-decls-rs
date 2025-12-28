macro_rules! deps {
    () => {
        Dependency!();
        WorkspacePackage!();
    };
}

macro_rules! Workspace {
    () => {
        deps!();
        # [derive (Serialize , Debug)] pub struct Workspace { # [serde (skip_serializing_if = "WorkspacePackage::is_none")] pub package : WorkspacePackage , # [serde (skip_serializing_if = "Map::is_empty")] pub dependencies : Map < String , Dependency > , }
    };
}

Workspace!()