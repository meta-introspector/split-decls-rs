macro_rules! LinuxInfo {
    () => {
        # [derive (Debug , Clone)] pub struct LinuxInfo { pub kernel_version : String , pub architecture : String , }
    };
}

LinuxInfo!();