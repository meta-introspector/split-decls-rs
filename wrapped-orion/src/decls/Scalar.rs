macro_rules! Scalar {
    () => {
        # [derive (Clone)] # [doc = " Represents a Scalar decoded from a byte array."] struct Scalar ([u8 ; PRIVATE_KEY_SIZE]) ;
    };
}

Scalar!()