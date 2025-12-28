macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! VersionMinCommand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct VersionMinCommand < E : Endian > { # [doc = " LC_VERSION_MIN_MACOSX or LC_VERSION_MIN_IPHONEOS or LC_VERSION_MIN_WATCHOS or LC_VERSION_MIN_TVOS"] pub cmd : U32 < E > , # [doc = " sizeof(struct VersionMinCommand)"] pub cmdsize : U32 < E > , # [doc = " X.Y.Z is encoded in nibbles xxxx.yy.zz"] pub version : U32 < E > , # [doc = " X.Y.Z is encoded in nibbles xxxx.yy.zz"] pub sdk : U32 < E > , }
    };
}

VersionMinCommand!();