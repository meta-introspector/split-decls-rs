macro_rules! deps {
    () => {
        IncrementCounter!();
        CVWords!();
    };
}

macro_rules! HashManyFn {
    () => {
        deps!();
        type HashManyFn < A > = unsafe fn (inputs : & [& A] , key : & CVWords , counter : u64 , increment_counter : IncrementCounter , flags : u8 , flags_start : u8 , flags_end : u8 , out : & mut [u8] ,) ;
    };
}

HashManyFn!();