macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! VersionDef {
    () => {
        deps!();
        # [doc = " A GNU version definition."] # [derive (Debug)] pub struct VersionDef < 'data > { # [doc = " The names for the version."] # [doc = ""] # [doc = " This usually has two elements. The first element is the name of this"] # [doc = " version, and the second element is the name of the previous version"] # [doc = " in the tree of versions."] pub names : Vec < ByteString < 'data > > , # [doc = " The version flags."] # [doc = ""] # [doc = " A combination of the `VER_FLG_*` constants."] pub flags : u16 , }
    };
}

VersionDef!();