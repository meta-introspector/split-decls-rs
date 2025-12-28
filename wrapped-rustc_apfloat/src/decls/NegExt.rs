macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! NegExt {
    () => {
        deps!();
        trait NegExt : Neg < Output = Self > + Sized { fn negate_if (self , negate : bool) -> Self { if negate { - self } else { self } } fn with_sign (self , sign : bool) -> Self where Self : Float , { self . negate_if (self . is_negative () != sign) } }
    };
}

NegExt!()