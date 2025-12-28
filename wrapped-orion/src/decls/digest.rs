macro_rules! deps {
    () => {
        UnknownCryptoError!();
        Hasher!();
    };
}

macro_rules! digest {
    () => {
        deps!();
        # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Hashing using BLAKE2b-256."] pub fn digest (data : & [u8]) -> Result < Digest , UnknownCryptoError > { blake2b :: Hasher :: Blake2b256 . digest (data) }
    };
}

digest!();