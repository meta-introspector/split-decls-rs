macro_rules! deps {
    () => {
        NonfiniteBehavior!();
        IeeeFloat!();
        Category!();
        Semantics!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < S : Semantics > IeeeFloat < S > { const fn qnan (payload : Option < u128 >) -> Self { let sig = [S :: NAN_SIGNIFICAND_BASE | S :: QNAN_SIGNIFICAND | match payload { Some (payload) => payload & S :: NAN_PAYLOAD_MASK , None => 0 , }] ; let exp = match S :: NONFINITE_BEHAVIOR { NonfiniteBehavior :: IEEE754 => S :: MAX_EXP + 1 , NonfiniteBehavior :: NanOnly => S :: MAX_EXP , } ; IeeeFloat { sig , exp , read_only_category_do_not_mutate : Category :: NaN , read_only_sign_do_not_mutate : false , marker : PhantomData , } } }
    };
}

impl_44!();