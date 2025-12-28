macro_rules! RustDetailsInfo {
    () => {
        # [derive (Debug , Clone)] pub struct RustDetailsInfo { pub version : String , pub crate_name : String , pub item_path : String , }
    };
}

RustDetailsInfo!();