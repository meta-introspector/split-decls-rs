macro_rules! deps {
    () => {
        NestedProgress!();
        DynNestedProgress!();
    };
}

macro_rules! DynNestedProgressToNestedProgress {
    () => {
        deps!();
        # [doc = " A bridge type that implements [`NestedProgress`] for any type that implements [`DynNestedProgress`]."] pub struct DynNestedProgressToNestedProgress < T : ? Sized > (pub T) ;
    };
}

DynNestedProgressToNestedProgress!()