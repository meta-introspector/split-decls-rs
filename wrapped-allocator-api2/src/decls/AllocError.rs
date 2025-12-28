macro_rules! AllocError {
    () => {
        # [doc = " The `AllocError` error indicates an allocation failure"] # [doc = " that may be due to resource exhaustion or to"] # [doc = " something wrong when combining the given input arguments with this"] # [doc = " allocator."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub struct AllocError ;
    };
}

AllocError!()