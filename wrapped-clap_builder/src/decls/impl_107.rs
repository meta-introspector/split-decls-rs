macro_rules! deps {
    () => {
        Str!();
        OsStr!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < Str > for OsStr { fn from (id : Str) -> Self { match id . into_inner () { crate :: builder :: StrInner :: Static (s) => Self :: from_static_ref (std :: ffi :: OsStr :: new (s)) , crate :: builder :: StrInner :: Owned (s) => Self :: from_ref (std :: ffi :: OsStr :: new (s . as_ref ())) , } } }
    };
}

impl_107!()