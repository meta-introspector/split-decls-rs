macro_rules! MachOBuildVersion {
    () => {
        # [doc = " The customizable portion of a [`macho::BuildVersionCommand`]."] # [derive (Debug , Default , Clone , Copy)] # [non_exhaustive] pub struct MachOBuildVersion { # [doc = " One of the `PLATFORM_` constants (for example,"] # [doc = " [`object::macho::PLATFORM_MACOS`](macho::PLATFORM_MACOS))."] pub platform : u32 , # [doc = " The minimum OS version, where `X.Y.Z` is encoded in nibbles as"] # [doc = " `xxxx.yy.zz`."] pub minos : u32 , # [doc = " The SDK version as `X.Y.Z`, where `X.Y.Z` is encoded in nibbles as"] # [doc = " `xxxx.yy.zz`."] pub sdk : u32 , }
    };
}

MachOBuildVersion!()