macro_rules! SignatureSize {
    () => {
        # [doc = " Size of a fixed sized signature for the given elliptic curve."] pub type SignatureSize < C > = < FieldBytesSize < C > as Add > :: Output ;
    };
}

SignatureSize!()