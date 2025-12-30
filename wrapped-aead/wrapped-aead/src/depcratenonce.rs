// Generated macro for Nonce (type)
macro_rules! DepcrateNonce {
() => {
// Module: crate
// Provides: {"Nonce"}
// Dependencies: {}
# [doc = " Nonce: single-use value for ensuring ciphertexts are unique"] pub type Nonce < A > = Array < u8 , < A as AeadCore > :: NonceSize > ;
};
}
