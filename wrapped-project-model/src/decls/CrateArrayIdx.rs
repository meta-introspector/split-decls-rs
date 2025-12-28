macro_rules! deps {
    () => {
        Crate!();
        ProjectJson!();
    };
}

macro_rules! CrateArrayIdx {
    () => {
        deps!();
        # [doc = " Identifies a crate by position in the crates array."] # [doc = ""] # [doc = " This will differ from `Crate` when multiple `ProjectJson`"] # [doc = " workspaces are loaded."] # [derive (Serialize , Deserialize , Debug , Clone , Copy , Eq , PartialEq , Hash)] # [serde (transparent)] pub struct CrateArrayIdx (pub usize) ;
    };
}

CrateArrayIdx!()