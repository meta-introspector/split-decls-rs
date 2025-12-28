macro_rules! deps {
    () => {
        Prepare!();
    };
}

macro_rules! impl_955 {
    () => {
        deps!();
        impl < T > Prepare < '_ , '_ , T > where T : Transport , { # [doc = " Return the `ref_map` (that includes the server handshake) which was part of listing refs prior to fetching a pack."] pub fn ref_map (& self) -> & RefMap { & self . ref_map } }
    };
}

impl_955!();