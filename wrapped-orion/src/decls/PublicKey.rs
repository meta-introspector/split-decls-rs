macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! PublicKey {
    () => {
        deps!();
        # [allow (clippy :: derive_partial_eq_without_eq)] # [doc = " A type that represents a `PublicKey` that X25519 uses."] # [doc = ""] # [doc = " This type holds a field element and is used internally as the u-coordinate."] # [doc = " As the RFC mandates, the most significant bit of the last byte is masked."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] # [derive (PartialEq , Debug , Clone)] pub struct PublicKey { fe : FieldElement , }
    };
}

PublicKey!();