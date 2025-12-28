macro_rules! deps {
    () => {
        IntrusiveArrayConsumer!();
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! ArrayConsumer {
    () => {
        deps!();
        # [doc = " **UNSAFE**: Consumes an array one element at a time."] # [doc = ""] # [doc = " You MUST increment the position while iterating and any leftover elements"] # [doc = " will be dropped if position does not go to N"] # [doc = ""] # [doc = " This is soft-deprecated in favor of [`IntrusiveArrayConsumer`] due to Rust's"] # [doc = " lack of return-value optimization causing issues moving the array in/out of the struct."] pub struct ArrayConsumer < T , N : ArrayLength > { array : ManuallyDrop < GenericArray < T , N > > , position : usize , }
    };
}

ArrayConsumer!();