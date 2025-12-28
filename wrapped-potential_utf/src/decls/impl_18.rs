macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        # [doc = " This impl requires enabling the optional `databake` Cargo feature"] # [cfg (feature = "databake")] impl databake :: Bake for PotentialCodePoint { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { match self . try_to_char () { Ok (ch) => { env . insert ("potential_utf") ; let ch = ch . bake (env) ; databake :: quote ! { potential_utf :: PotentialCodePoint :: from_char (# ch) } } Err (_) => { env . insert ("potential_utf") ; let u24 = u32 :: from_le_bytes ([self . 0 [0] , self . 0 [1] , self . 0 [2] , 0]) ; databake :: quote ! { potential_utf :: PotentialCodePoint :: from_u24 (# u24) } } } } }
    };
}

impl_18!();