macro_rules! AlignFromBytesError {
    () => {
        # [derive (Clone , Copy)] pub enum AlignFromBytesError { NotPowerOfTwo (u64) , TooLarge (u64) , }
    };
}

AlignFromBytesError!();