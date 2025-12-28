macro_rules! Patch {
    () => {
        # [derive (Serialize , Deserialize , Clone , Debug)] pub struct Patch { # [serde (skip_serializing_if = "Option::is_none")] pub path : Option < PathBuf > , # [serde (skip_serializing_if = "Option::is_none")] pub git : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub branch : Option < String > , }
    };
}

Patch!();