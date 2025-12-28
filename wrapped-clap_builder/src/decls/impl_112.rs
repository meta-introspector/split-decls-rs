macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < String > for OsStr { fn from (name : String) -> Self { Self :: from_string (name . into ()) } }
    };
}

impl_112!();