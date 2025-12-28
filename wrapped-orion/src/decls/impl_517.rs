macro_rules! deps {
    () => {
        MlKem768Internal!();
        DecapsulationKey!();
        UnknownCryptoError!();
        EncapsulationKey!();
        MlKem768!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        impl MlKem768 { # [doc = " Encapsulation key size (bytes)."] pub const EK_SIZE : usize = MlKem768Internal :: EK_SIZE ; # [doc = " Decapsulation key size (bytes)."] pub const DK_SIZE : usize = MlKem768Internal :: DK_SIZE ; # [doc = " Ciphertext size (bytes)."] pub const CIPHERTEXT_SIZE : usize = MlKem768Internal :: CIPHERTEXT_SIZE ; # [doc = " Shared Secret size (bytes)."] pub const SHARED_SECRET_SIZE : usize = MlKem768Internal :: SHARED_SECRET_SIZE ; # [cfg (feature = "safe_api")] # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " Given the [EncapsulationKey], generate a [SharedSecret] and associated [Ciphertext]."] pub fn encap (ek : & EncapsulationKey) -> Result < (SharedSecret , Ciphertext) , UnknownCryptoError > { ek . encap () } # [doc = " Given the [DecapsulationKey], produce a [SharedSecret] using the [Ciphertext]."] pub fn decap (dk : & DecapsulationKey , c : & Ciphertext ,) -> Result < SharedSecret , UnknownCryptoError > { dk . decap (c) } }
    };
}

impl_517!()