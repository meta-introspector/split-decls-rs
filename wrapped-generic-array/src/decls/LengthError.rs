macro_rules! LengthError {
    () => {
        # [doc = " Error type for [`TryFrom`] and [`try_from_iter`](GenericArray::try_from_iter) implementations."] # [derive (Debug , Clone , Copy)] pub struct LengthError ;
    };
}

LengthError!();