macro_rules! deps {
    () => {
        Shared!();
        Inner!();
    };
}

macro_rules! WeakShared {
    () => {
        deps!();
        # [doc = " A weak reference to a [`Shared`] that can be upgraded much like an `Arc`."] pub struct WeakShared < Fut : Future > (Weak < Inner < Fut > >) ;
    };
}

WeakShared!();