macro_rules! deps {
    () => {
        Error!();
        BernoulliError!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for BernoulliError { }
    };
}

impl_12!()