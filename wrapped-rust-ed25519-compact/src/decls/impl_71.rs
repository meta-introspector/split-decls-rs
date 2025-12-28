macro_rules! deps {
    () => {
        Error!();
        Signature!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl Signature { # [doc = " Number of raw bytes in a signature."] pub const BYTES : usize = 64 ; # [doc = " Creates a signature from raw bytes."] pub fn new (bytes : [u8 ; Signature :: BYTES]) -> Self { Signature (bytes) } # [doc = " Creates a signature key from a slice."] pub fn from_slice (signature : & [u8]) -> Result < Self , Error > { let mut signature_ = [0u8 ; Signature :: BYTES] ; if signature . len () != signature_ . len () { return Err (Error :: InvalidSignature) ; } signature_ . copy_from_slice (signature) ; Ok (Signature :: new (signature_)) } }
    };
}

impl_71!()