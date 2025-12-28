macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! derive_key {
    () => {
        deps!();
        # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Derive a key using Argon2i."] pub fn derive_key (password : & Password , salt : & Salt , iterations : u32 , memory : u32 , length : u32 ,) -> Result < SecretKey , UnknownCryptoError > { if iterations < MIN_ITERATIONS { return Err (UnknownCryptoError) ; } let mut dk = SecretKey :: from_slice (& vec ! [0u8 ; length as usize]) ? ; argon2i :: derive_key (password . unprotected_as_bytes () , salt . as_ref () , iterations , memory , None , None , & mut dk . value ,) ? ; Ok (dk) }
    };
}

derive_key!()