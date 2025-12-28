macro_rules! deps {
    () => {
        PeekingTakeWhile!();
    };
}

macro_rules! impl_410 {
    () => {
        deps!();
        impl < 'a , I , F > std :: fmt :: Debug for PeekingTakeWhile < 'a , I , F > where I : Iterator + std :: fmt :: Debug + 'a , { debug_fmt_fields ! (PeekingTakeWhile , iter) ; }
    };
}

impl_410!()