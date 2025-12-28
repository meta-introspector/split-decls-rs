macro_rules! deps {
    () => {
        EncapsulationKey!();
    };
}

macro_rules! macro_545 {
    () => {
        deps!();
        construct_public ! { # [doc = " A type to represent the public `EncapsulationKey` that X-Wing uses."] # [doc = ""] # [doc = " This type simply holds bytes and performs no checks whatsoever. If an invalid"] # [doc = " ML-KEM-768 is part of the bytes parsed from this type, the check will first surface"] # [doc = " when encapsulation is performed."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 1216 bytes."] (EncapsulationKey , test_kem_encapkey , PUBLIC_KEY_SIZE , PUBLIC_KEY_SIZE) }
    };
}

macro_545!();