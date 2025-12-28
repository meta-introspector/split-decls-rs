macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! HmacFunction {
    () => {
        deps!();
        # [doc = " A trait used to define a HMAC function."] pub (crate) trait HmacFunction { # [doc = " The output size of the internal hash function used."] const HASH_FUNC_OUTSIZE : usize ; # [doc = " Create a new instance of the HMAC function, using a `secret_key` that may or may not be padded."] fn _new (secret_key : & [u8]) -> Result < Self , UnknownCryptoError > where Self : Sized ; # [doc = " Update the internal state with `data`."] fn _update (& mut self , data : & [u8]) -> Result < () , UnknownCryptoError > ; # [doc = " Finalize the MAC and put the final tag into `dest`."] # [doc = ""] # [doc = " NOTE: `dest` may be less than the complete output size of the hash function"] # [doc = " (Self::HASH_FUNC_OUTSIZE). If that is the case, `dest.len()` bytes will be copied,"] # [doc = " but `dest` should NEVER be empty."] fn _finalize (& mut self , dest : & mut [u8]) -> Result < () , UnknownCryptoError > ; # [doc = " Reset the state."] fn _reset (& mut self) ; }
    };
}

HmacFunction!()