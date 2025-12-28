macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < String > for Str { fn from (name : String) -> Self { Self :: from_string (name) } }
    };
}

impl_195!();