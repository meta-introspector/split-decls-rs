macro_rules! TargetKindData {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Deserialize , Serialize)] # [serde (rename_all = "camelCase")] pub enum TargetKindData { Bin , # [doc = " Any kind of Cargo lib crate-type (dylib, rlib, proc-macro, ...)."] Lib , Test , }
    };
}

TargetKindData!();