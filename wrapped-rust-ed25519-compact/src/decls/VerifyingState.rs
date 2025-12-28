macro_rules! deps {
    () => {
        GeP3!();
        Signature!();
        Hash!();
    };
}

macro_rules! VerifyingState {
    () => {
        deps!();
        # [doc = " The state of a streaming verification operation."] # [derive (Clone)] pub struct VerifyingState { hasher : sha512 :: Hash , signature : Signature , a : GeP3 , }
    };
}

VerifyingState!()