macro_rules! deps {
    () => {
        TargetKindData!();
    };
}

macro_rules! BuildData {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize , Eq , PartialEq)] pub struct BuildData { label : String , build_file : Utf8PathBuf , target_kind : TargetKindData , }
    };
}

BuildData!()