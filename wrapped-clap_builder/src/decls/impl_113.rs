macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < & '_ String > for OsStr { fn from (name : & '_ String) -> Self { Self :: from_ref (name . as_str () . as_ref ()) } }
    };
}

impl_113!();