macro_rules! deps {
    () => {
        Bag!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        # [doc = " `Bag::try_push()` requires that it is safe for another thread to execute the given functions."] unsafe impl Send for Bag { }
    };
}

impl_102!()