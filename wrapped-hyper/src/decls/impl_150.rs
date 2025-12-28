macro_rules! deps {
    () => {
        Protocol!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        # [cfg (feature = "http2")] impl Protocol { # [doc = " Converts a static string to a protocol name."] pub const fn from_static (value : & 'static str) -> Self { Self { inner : h2 :: ext :: Protocol :: from_static (value) , } } # [doc = " Returns a str representation of the header."] pub fn as_str (& self) -> & str { self . inner . as_str () } # [cfg (feature = "server")] pub (crate) fn from_inner (inner : h2 :: ext :: Protocol) -> Self { Self { inner } } # [cfg (all (feature = "client" , feature = "http2"))] pub (crate) fn into_inner (self) -> h2 :: ext :: Protocol { self . inner } }
    };
}

impl_150!()