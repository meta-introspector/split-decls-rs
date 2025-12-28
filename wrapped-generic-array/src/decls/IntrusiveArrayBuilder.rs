macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
        ArrayBuilder!();
    };
}

macro_rules! IntrusiveArrayBuilder {
    () => {
        deps!();
        # [doc = " Similar to [`ArrayBuilder`] but uses a reference to a pre-allocated array, be"] # [doc = " it on the stack or heap."] pub struct IntrusiveArrayBuilder < 'a , T , N : ArrayLength > { array : & 'a mut GenericArray < MaybeUninit < T > , N > , position : usize , }
    };
}

IntrusiveArrayBuilder!()