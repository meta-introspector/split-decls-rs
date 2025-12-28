macro_rules! deps {
    () => {
        SigningState!();
        Signature!();
        Hash!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl SigningState { fn new (nonce : [u8 ; 64] , az : [u8 ; 64] , pk_ : & [u8]) -> Self { let mut prefix : [u8 ; 64] = [0 ; 64] ; let r = ge_scalarmult_base (& nonce [0 .. 32]) ; prefix [0 .. 32] . copy_from_slice (& r . to_bytes () [..]) ; prefix [32 .. 64] . copy_from_slice (pk_) ; let mut st = sha512 :: Hash :: new () ; st . update (prefix) ; SigningState { hasher : st , nonce , az , } } # [doc = " Appends data to the message being signed."] pub fn absorb (& mut self , chunk : impl AsRef < [u8] >) { self . hasher . update (chunk) } # [doc = " Computes the signature and return it."] pub fn sign (& self) -> Signature { let mut signature : [u8 ; 64] = [0 ; 64] ; let r = ge_scalarmult_base (& self . nonce [0 .. 32]) ; signature [0 .. 32] . copy_from_slice (& r . to_bytes () [..]) ; let mut hram = self . hasher . finalize () ; sc_reduce (& mut hram) ; sc_muladd (& mut signature [32 .. 64] , & hram [0 .. 32] , & self . az [0 .. 32] , & self . nonce [0 .. 32] ,) ; Signature (signature) } }
    };
}

impl_80!()