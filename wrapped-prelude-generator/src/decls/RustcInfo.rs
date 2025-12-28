macro_rules! RustcInfo {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct RustcInfo { pub version : String , pub host : String , }
    };
}

RustcInfo!()