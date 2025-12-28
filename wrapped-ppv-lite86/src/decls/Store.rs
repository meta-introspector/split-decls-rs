macro_rules! Store {
    () => {
        pub trait Store < S > { # [doc = " # Safety"] # [doc = " Caller must ensure the type of Self is appropriate for the hardware of the execution"] # [doc = " environment."] unsafe fn unpack (p : S) -> Self ; }
    };
}

Store!();