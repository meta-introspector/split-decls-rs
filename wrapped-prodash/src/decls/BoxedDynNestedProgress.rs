macro_rules! deps {
    () => {
        DynNestedProgress!();
    };
}

macro_rules! BoxedDynNestedProgress {
    () => {
        deps!();
        # [doc = " An opaque type for storing [`DynNestedProgress`]."] pub struct BoxedDynNestedProgress (Box < dyn DynNestedProgress >) ;
    };
}

BoxedDynNestedProgress!();