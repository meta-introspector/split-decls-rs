macro_rules! deps {
    () => {
        PrivateKey!();
        PublicKey!();
        UnknownCryptoError!();
    };
}

macro_rules! key_agreement {
    () => {
        deps!();
        # [doc = " X25519 (Diffie-Hellman with Montgomery form of Curve25519)."] pub fn key_agreement (private_key : & PrivateKey , public_key : & PublicKey ,) -> Result < SharedKey , UnknownCryptoError > { let u_coord = public_key . fe ; let field_element = mont_ladder (& private_key . scalar , u_coord) . as_bytes () ; debug_assert_eq ! (field_element [31] & 0b1000_0000u8 , 0u8) ; if secure_cmp (& field_element , & LOW_ORDER_POINT_RESULT) . is_ok () { return Err (UnknownCryptoError) ; } Ok (SharedKey :: from (field_element)) }
    };
}

key_agreement!();