macro_rules! deps {
    () => {
        UnknownCryptoError!();
        Blake2b!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl Blake2b { # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Initialize a `Blake2b` struct with a given size (in bytes)."] pub fn new (size : usize) -> Result < Self , UnknownCryptoError > { Ok (Self { _state : blake2b_core :: State :: _new (& [] , size) ? , }) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Reset to `new()` state."] pub fn reset (& mut self) -> Result < () , UnknownCryptoError > { self . _state . _reset (& []) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Update state with `data`. This can be called multiple times."] pub fn update (& mut self , data : & [u8]) -> Result < () , UnknownCryptoError > { self . _state . _update (data) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Return a BLAKE2b digest."] pub fn finalize (& mut self) -> Result < Digest , UnknownCryptoError > { let mut tmp = [0u8 ; BLAKE2B_OUTSIZE] ; self . _state . _finalize (& mut tmp) ? ; Digest :: from_slice (& tmp [.. self . _state . size]) } }
    };
}

impl_232!()