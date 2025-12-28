macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! IntrusiveArrayConsumer {
    () => {
        deps!();
        # [doc = " **UNSAFE**: Consumes an array one element at a time."] # [doc = ""] # [doc = " You MUST increment the position while iterating and any leftover elements"] # [doc = " will be dropped if position does not go to N"] pub struct IntrusiveArrayConsumer < 'a , T , N : ArrayLength > { array : & 'a mut ManuallyDrop < GenericArray < T , N > > , position : usize , }
    };
}

IntrusiveArrayConsumer!();