macro_rules! TestInfo {
    () => {
        # [derive (Debug , Serialize , Deserialize , PartialEq , Eq , Hash , Clone)] pub struct TestInfo { pub name : String , pub file_path : PathBuf , }
    };
}

TestInfo!()