macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl Str { # [cfg (feature = "string")] pub (crate) fn from_string (name : String) -> Self { Self { name : Inner :: from_string (name) , } } # [cfg (feature = "string")] pub (crate) fn from_ref (name : & str) -> Self { Self { name : Inner :: from_ref (name) , } } pub (crate) fn from_static_ref (name : & 'static str) -> Self { Self { name : Inner :: from_static_ref (name) , } } pub (crate) fn into_inner (self) -> Inner { self . name } # [doc = " Get the raw string of the `Str`"] pub fn as_str (& self) -> & str { self . name . as_str () } }
    };
}

impl_193!()