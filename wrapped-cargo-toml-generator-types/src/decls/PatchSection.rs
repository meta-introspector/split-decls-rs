macro_rules! deps {
    () => {
        Dependency!();
    };
}

macro_rules! PatchSection {
    () => {
        deps!();
        # [derive (Debug , Default , Clone , PartialEq , Serialize , Deserialize)] pub struct PatchSection { # [serde (rename = "crates-io" , default , skip_serializing_if = "HashMap::is_empty")] pub crates_io : HashMap < String , Dependency > , }
    };
}

PatchSection!();