macro_rules! CargoInfo {
    () => {
        # [derive (Debug , Clone)] pub struct CargoInfo { pub package_name : String , pub version : String , }
    };
}

CargoInfo!()