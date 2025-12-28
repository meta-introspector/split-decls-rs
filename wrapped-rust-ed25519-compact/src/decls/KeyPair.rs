macro_rules! deps {
    () => {
        SecretKey!();
        Hash!();
        PublicKey!();
    };
}

macro_rules! KeyPair {
    () => {
        deps!();
        # [doc = " A key pair."] # [derive (Clone , Debug , Eq , PartialEq , Hash)] pub struct KeyPair { # [doc = " Public key part of the key pair."] pub pk : PublicKey , # [doc = " Secret key part of the key pair."] pub sk : SecretKey , }
    };
}

KeyPair!()