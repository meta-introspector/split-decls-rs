macro_rules! MH_SIM_SUPPORT {
    () => {
        # [doc = " Allow LC_MIN_VERSION_MACOS and LC_BUILD_VERSION load commands with"] # [doc = " the platforms macOS, iOSMac, iOSSimulator, tvOSSimulator and watchOSSimulator."] pub const MH_SIM_SUPPORT : u32 = 0x0800_0000 ;
    };
}

MH_SIM_SUPPORT!();