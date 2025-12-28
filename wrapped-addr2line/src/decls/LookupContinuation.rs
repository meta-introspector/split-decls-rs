macro_rules! deps {
    () => {
        LookupResult!();
    };
}

macro_rules! LookupContinuation {
    () => {
        deps!();
        # [doc = " This trait represents a partially complete operation that can be resumed"] # [doc = " once a load of needed split DWARF data is completed or abandoned by the"] # [doc = " API consumer."] pub trait LookupContinuation : Sized { # [doc = " The final output of this operation."] type Output ; # [doc = " The type of reader used."] type Buf : gimli :: Reader ; # [doc = " Resumes the operation with the provided data."] # [doc = ""] # [doc = " After the caller loads the split DWARF data required, call this"] # [doc = " method to resume the operation. The return value of this method"] # [doc = " indicates if the computation has completed or if further data is"] # [doc = " required."] # [doc = ""] # [doc = " If the additional data cannot be located, or the caller does not"] # [doc = " support split DWARF, `resume(None)` can be used to continue the"] # [doc = " operation with the data that is available."] fn resume (self , input : Option < Arc < gimli :: Dwarf < Self :: Buf > > >) -> LookupResult < Self > ; }
    };
}

LookupContinuation!()