macro_rules! deps {
    () => {
        ByteString!();
        VersionFileId!();
    };
}

macro_rules! VersionNeed {
    () => {
        deps!();
        # [doc = " A GNU version dependency."] # [derive (Debug)] pub struct VersionNeed < 'data > { # [doc = " The filename of the library providing this version."] pub file : VersionFileId , # [doc = " The name of the version."] pub name : ByteString < 'data > , # [doc = " The version flags."] # [doc = ""] # [doc = " A combination of the `VER_FLG_*` constants."] pub flags : u16 , }
    };
}

VersionNeed!()