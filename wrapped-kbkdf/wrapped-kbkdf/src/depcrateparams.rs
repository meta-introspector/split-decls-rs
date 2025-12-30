// Generated macro for Params (struct)
macro_rules! DepcrateParams {
() => {
// Module: crate
// Provides: {"Params"}
// Dependencies: {}
# [doc = " Parameters used for KBKDF."] # [doc = ""] # [doc = " For more details, read the official specification: [NIST SP 800-108r1](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-108r1.pdf)."] pub struct Params < 'k , 'l , 'c > { # [doc = " Key-derivation key."] # [doc = ""] # [doc = " key that is used as an input to a key-derivation function (along with other input data) to derive keying material."] pub kin : & 'k [u8] , # [doc = " A string that identifies the purpose for the derived keying material, which is encoded as a bit string."] # [doc = ""] # [doc = " The encoding method for the Label is defined in a larger context, for example, in the protocol that uses a KDF."] pub label : & 'l [u8] , # [doc = " A bit string containing the information related to the derived keying material."] # [doc = ""] # [doc = " It may include the identities of the parties who are deriving and/or using the derived keying material and,"] # [doc = " optionally, a nonce known by the parties who derive the keys."] pub context : & 'c [u8] , # [doc = " A flag indicating whether to update the Prf with the requested key length."] pub use_l : bool , # [doc = " A flag indicating whether to separate the label from the context with a NULL byte."] pub use_separator : bool , # [doc = " A flag indicating whether to update the Prf with the iteration counter."] pub use_counter : bool , }
};
}
