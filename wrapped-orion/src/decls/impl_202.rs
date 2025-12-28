macro_rules! deps {
    () => {
        UnknownCryptoError!();
        Shake128!();
        Shake!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl Shake128 { # [doc = " Initialize a `Shake128` struct."] pub fn new () -> Self { Self { _state : Shake :: < SHAKE_128_RATE > :: _new (32) , } } # [doc = " Reset to `new()` state."] pub fn reset (& mut self) { self . _state . _reset () ; } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Update state with `data`. This can be called multiple times."] pub fn absorb (& mut self , data : & [u8]) -> Result < () , UnknownCryptoError > { self . _state . _absorb (data) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Squeeze output of the XOF into `dest`. This can be called multiple times."] pub fn squeeze (& mut self , dest : & mut [u8]) -> Result < () , UnknownCryptoError > { self . _state . _squeeze (dest) } }
    };
}

impl_202!()