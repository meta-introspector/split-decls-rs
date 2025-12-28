macro_rules! WorkspacePackage {
    () => {
        # [derive (Serialize , Debug)] pub struct WorkspacePackage { # [serde (skip_serializing_if = "Option::is_none")] pub edition : Option < String > , }
    };
}

WorkspacePackage!()