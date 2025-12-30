// Generated macro for impl_679 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem768impl_679 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem768
// Provides: {"impl_679"}
// Dependencies: {}
impl MlKem768 { # [doc = " Encapsulation key size (bytes)."] pub const EK_SIZE : usize = MlKem768Internal :: EK_SIZE ; # [doc = " Decapsulation key size (bytes)."] pub const DK_SIZE : usize = MlKem768Internal :: DK_SIZE ; # [doc = " Ciphertext size (bytes)."] pub const CIPHERTEXT_SIZE : usize = MlKem768Internal :: CIPHERTEXT_SIZE ; # [doc = " Shared Secret size (bytes)."] pub const SHARED_SECRET_SIZE : usize = MlKem768Internal :: SHARED_SECRET_SIZE ; # [cfg (feature = "safe_api")] # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " Given the [EncapsulationKey], generate a [SharedSecret] and associated [Ciphertext]."] pub fn encap (ek : & EncapsulationKey) -> Result < (SharedSecret , Ciphertext) , UnknownCryptoError > { ek . encap () } # [doc = " Given the [DecapsulationKey], produce a [SharedSecret] using the [Ciphertext]."] pub fn decap (dk : & DecapsulationKey , c : & Ciphertext ,) -> Result < SharedSecret , UnknownCryptoError > { dk . decap (c) } }
};
}
