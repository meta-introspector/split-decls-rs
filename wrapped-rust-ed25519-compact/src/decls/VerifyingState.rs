macro_rules! deps {
    () => {
        Signature!();
        Hash!();
        GeP3!();
    };
}

macro_rules! VerifyingState {
    () => {
        deps!();
        # [doc = " The state of a streaming verification operation."] # [derive (Clone)] pub struct VerifyingState { hasher : sha512 :: Hash , signature : Signature , a : GeP3 , }
    };
}

VerifyingState!();