macro_rules! deps {
    () => {
        SignatureSize!();
    };
}

macro_rules! SignatureBytes {
    () => {
        deps!();
        # [doc = " Fixed-size byte array containing an ECDSA signature"] pub type SignatureBytes < C > = Array < u8 , SignatureSize < C > > ;
    };
}

SignatureBytes!()