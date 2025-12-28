macro_rules! NixInfo {
    () => {
        # [derive (Debug , Clone)] pub struct NixInfo { pub flake_path : String , pub output_type : String , }
    };
}

NixInfo!();