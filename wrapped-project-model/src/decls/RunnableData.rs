macro_rules! deps {
    () => {
        RunnableKindData!();
    };
}

macro_rules! RunnableData {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] pub struct RunnableData { pub program : String , pub args : Vec < String > , pub cwd : Utf8PathBuf , pub kind : RunnableKindData , }
    };
}

RunnableData!()