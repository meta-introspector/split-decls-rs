macro_rules! deps {
    () => {
        CapacityError!();
        CString!();
    };
}

macro_rules! ExtendError {
    () => {
        deps!();
        # [doc = " An error to extend [`CString`] with bytes."] # [derive (Debug)] pub enum ExtendError { # [doc = " The capacity of the [`CString`] is too small."] Capacity (CapacityError) , # [doc = " An invalid interior nul byte found in a given byte slice."] InteriorNul { # [doc = " A position of a nul byte."] position : usize , } , }
    };
}

ExtendError!();