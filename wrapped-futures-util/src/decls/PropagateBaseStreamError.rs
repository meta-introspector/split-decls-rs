macro_rules! PropagateBaseStreamError {
    () => {
        # [doc = " Immediately propagates errors occurred in the base stream."] # [derive (Debug , Clone , Copy)] pub struct PropagateBaseStreamError < St > (PhantomData < St >) ;
    };
}

PropagateBaseStreamError!();