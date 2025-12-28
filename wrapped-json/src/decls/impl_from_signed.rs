macro_rules! deps {
    () => {
        Number!();
    };
}

macro_rules! impl_from_signed {
    () => {
        deps!();
        macro_rules ! impl_from_signed { ($ ($ ty : ty) ,*) => { $ (impl From <$ ty > for Number { fn from (i : $ ty) -> Self { let n = { # [cfg (not (feature = "arbitrary_precision"))] { if i < 0 { N :: NegInt (i as i64) } else { N :: PosInt (i as u64) } } # [cfg (feature = "arbitrary_precision")] { itoa :: Buffer :: new () . format (i) . to_owned () } } ; Number { n } } }) * } ; }
    };
}

impl_from_signed!()