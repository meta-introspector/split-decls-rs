macro_rules! deps {
    () => {
        EucJpPending!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl EucJpPending { fn is_none (& self) -> bool { match * self { EucJpPending :: None => true , _ => false , } } fn count (& self) -> usize { match * self { EucJpPending :: None => 0 , EucJpPending :: Jis0208Lead (_) | EucJpPending :: Jis0212Shift | EucJpPending :: HalfWidthKatakana => 1 , EucJpPending :: Jis0212Lead (_) => 2 , } } }
    };
}

impl_70!()