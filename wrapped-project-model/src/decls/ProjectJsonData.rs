macro_rules! deps {
    () => {
        RunnableData!();
        CfgList!();
        CrateData!();
    };
}

macro_rules! ProjectJsonData {
    () => {
        deps!();
        # [derive (Serialize , Deserialize , Debug , Clone , Eq , PartialEq)] pub struct ProjectJsonData { sysroot : Option < Utf8PathBuf > , sysroot_src : Option < Utf8PathBuf > , sysroot_project : Option < Box < ProjectJsonData > > , # [serde (default)] cfg_groups : FxHashMap < String , CfgList > , crates : Vec < CrateData > , # [serde (default)] runnables : Vec < RunnableData > , }
    };
}

ProjectJsonData!();