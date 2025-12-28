macro_rules! deps {
    () => {
        VerifyingState!();
        Error!();
        PublicKey!();
        Signature!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl PublicKey { # [doc = " Verify the signature of a multi-part message (streaming)."] pub fn verify_incremental (& self , signature : & Signature) -> Result < VerifyingState , Error > { VerifyingState :: new (self , signature) } # [doc = " Verifies that the signature `signature` is valid for the message"] # [doc = " `message`."] pub fn verify (& self , message : impl AsRef < [u8] > , signature : & Signature) -> Result < () , Error > { let mut st = VerifyingState :: new (self , signature) ? ; st . absorb (message) ; st . verify () } }
    };
}

impl_77!();