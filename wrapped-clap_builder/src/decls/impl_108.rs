macro_rules! deps {
    () => {
        Str!();
        OsStr!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        # [cfg (not (feature = "string"))] impl From < Str > for OsStr { fn from (id : Str) -> Self { Self :: from_static_ref (std :: ffi :: OsStr :: new (id . into_inner () . 0)) } }
    };
}

impl_108!();