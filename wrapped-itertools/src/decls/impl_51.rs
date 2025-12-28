macro_rules! deps {
    () => {
        MapSpecialCaseFnOk!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < F > std :: fmt :: Debug for MapSpecialCaseFnOk < F > { debug_fmt_fields ! (MapSpecialCaseFnOk ,) ; }
    };
}

impl_51!()