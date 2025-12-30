// Generated macro for agree_ephemeral (function)
macro_rules! Depcrate_agreementagree_ephemeral {
() => {
// Module: crate::agreement
// Provides: {"agree_ephemeral"}
// Dependencies: {}
# [doc = " Performs a key agreement with an ephemeral private key and the given public"] # [doc = " key."] # [doc = ""] # [doc = " `my_private_key` is the ephemeral private key to use. Since it is moved, it"] # [doc = " will not be usable after calling `agree_ephemeral`, thus guaranteeing that"] # [doc = " the key is used for only one key agreement."] # [doc = ""] # [doc = " `peer_public_key` is the peer's public key. `agree_ephemeral` will return"] # [doc = " `Err(error_value)` if it does not match `my_private_key's` algorithm/curve."] # [doc = " `agree_ephemeral` verifies that it is encoded in the standard form for the"] # [doc = " algorithm and that the key is *valid*; see the algorithm's documentation for"] # [doc = " details on how keys are to be encoded and what constitutes a valid key for"] # [doc = " that algorithm."] # [doc = ""] # [doc = " After the key agreement is done, `agree_ephemeral` calls `kdf` with the raw"] # [doc = " key material from the key agreement operation and then returns what `kdf`"] # [doc = " returns."] # [inline] pub fn agree_ephemeral < B : AsRef < [u8] > , R > (my_private_key : EphemeralPrivateKey , peer_public_key : & UnparsedPublicKey < B > , kdf : impl FnOnce (& [u8]) -> R ,) -> Result < R , error :: Unspecified > { let peer_public_key = UnparsedPublicKey { algorithm : peer_public_key . algorithm , bytes : peer_public_key . bytes . as_ref () , } ; agree_ephemeral_ (my_private_key , peer_public_key , kdf , cpu :: features ()) }
};
}
