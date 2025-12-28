macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! SigningState {
    () => {
        deps!();
        # [doc = " The state of a streaming signature operation."] # [derive (Clone)] pub struct SigningState { hasher : sha512 :: Hash , az : [u8 ; 64] , nonce : [u8 ; 64] , }
    };
}

SigningState!()