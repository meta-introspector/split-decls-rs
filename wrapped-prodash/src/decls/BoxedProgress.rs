macro_rules! deps {
    () => {
        Progress!();
    };
}

macro_rules! BoxedProgress {
    () => {
        deps!();
        # [doc = " An owned version of [`Progress`] which can itself implement said trait."] pub type BoxedProgress = Box < dyn Progress > ;
    };
}

BoxedProgress!()