macro_rules! deps {
    () => {
        Hash!();
        VerifyingState!();
        Error!();
        GeP3!();
        Signature!();
        PublicKey!();
        GeP2!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl VerifyingState { fn new (pk : & PublicKey , signature : & Signature) -> Result < Self , Error > { let r = & signature [0 .. 32] ; let s = & signature [32 .. 64] ; sc_reject_noncanonical (s) ? ; if is_identity (pk) || pk . iter () . fold (0 , | acc , x | acc | x) == 0 { return Err (Error :: WeakPublicKey) ; } let a = match GeP3 :: from_bytes_negate_vartime (pk) { Some (g) => g , None => { return Err (Error :: InvalidPublicKey) ; } } ; let mut hasher = sha512 :: Hash :: new () ; hasher . update (r) ; hasher . update (& pk [..]) ; Ok (VerifyingState { hasher , signature : * signature , a , }) } # [doc = " Appends data to the message being verified."] pub fn absorb (& mut self , chunk : impl AsRef < [u8] >) { self . hasher . update (chunk) } # [doc = " Verifies the signature and return it."] pub fn verify (& self) -> Result < () , Error > { let mut expected_r_bytes = [0u8 ; 32] ; expected_r_bytes . copy_from_slice (& self . signature [0 .. 32]) ; let expected_r = GeP3 :: from_bytes_vartime (& expected_r_bytes) . ok_or (Error :: InvalidSignature) ? ; let s = & self . signature [32 .. 64] ; let mut hash = self . hasher . finalize () ; sc_reduce (& mut hash) ; let r = GeP2 :: double_scalarmult_vartime (hash . as_ref () , self . a , s) ; if (expected_r - GeP3 :: from (r)) . has_small_order () { Ok (()) } else { Err (Error :: SignatureMismatch) } } }
    };
}

impl_76!();