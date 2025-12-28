macro_rules! deps {
    () => {
        Blake2b!();
        UnknownCryptoError!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl Blake2b { # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Initialize a `Blake2b` struct with a given size (in bytes) and key."] pub fn new (secret_key : & SecretKey , size : usize) -> Result < Self , UnknownCryptoError > { Ok (Self { _state : blake2b_core :: State :: _new (secret_key . unprotected_as_bytes () , size) ? , }) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Reset to `new()` state."] pub fn reset (& mut self , secret_key : & SecretKey) -> Result < () , UnknownCryptoError > { self . _state . _reset (secret_key . unprotected_as_bytes ()) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Update state with `data`. This can be called multiple times."] pub fn update (& mut self , data : & [u8]) -> Result < () , UnknownCryptoError > { self . _state . _update (data) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Return a BLAKE2b tag."] pub fn finalize (& mut self) -> Result < Tag , UnknownCryptoError > { let mut tmp : Zeroizing < [u8 ; BLAKE2B_OUTSIZE] > = Zeroizing :: new ([0u8 ; BLAKE2B_OUTSIZE]) ; self . _state . _finalize (tmp . deref_mut ()) ? ; Tag :: from_slice (& tmp [.. self . _state . size]) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Verify a BLAKE2b tag in constant time."] pub fn verify (expected : & Tag , secret_key : & SecretKey , size : usize , data : & [u8] ,) -> Result < () , UnknownCryptoError > { let mut ctx = Self :: new (secret_key , size) ? ; ctx . update (data) ? ; if & ctx . finalize () ? == expected { Ok (()) } else { Err (UnknownCryptoError) } } }
    };
}

impl_271!()