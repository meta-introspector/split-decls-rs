macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < & '_ String > for Str { fn from (name : & '_ String) -> Self { Self :: from_ref (name . as_str ()) } }
    };
}

impl_196!()