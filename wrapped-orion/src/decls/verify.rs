macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! verify {
    () => {
        deps!();
        # [allow (clippy :: too_many_arguments)] # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Verify Argon2i derived key in constant time."] pub fn verify (expected : & [u8] , password : & [u8] , salt : & [u8] , iterations : u32 , memory : u32 , secret : Option < & [u8] > , ad : Option < & [u8] > , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { derive_key (password , salt , iterations , memory , secret , ad , dst_out) ? ; util :: secure_cmp (dst_out , expected) }
    };
}

verify!()