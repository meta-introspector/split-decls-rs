macro_rules! deps {
    () => {
        X87DoubleExtendedS!();
        IeeeFloat!();
        IeeeDefaultExceptionHandling!();
        Semantics!();
        Category!();
        StatusAnd!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl IeeeDefaultExceptionHandling { fn result_from_nan < S : Semantics > (mut r : IeeeFloat < S >) -> StatusAnd < IeeeFloat < S > > { assert ! (r . is_nan ()) ; let status = if r . is_signaling () { let [sig] = & mut r . sig ; * sig |= if S :: QNAN_SIGNIFICAND == X87DoubleExtendedS :: QNAN_SIGNIFICAND { S :: QNAN_SIGNIFICAND & S :: NAN_PAYLOAD_MASK } else { S :: QNAN_SIGNIFICAND } ; Status :: INVALID_OP } else { Status :: OK } ; status . and (r) } fn binop_result_from_either_nan < S : Semantics > (a : IeeeFloat < S > , b : IeeeFloat < S >) -> StatusAnd < IeeeFloat < S > > { let r = match (a . category () , b . category ()) { (Category :: NaN , _) => a , (_ , Category :: NaN) => b , _ => unreachable ! () , } ; let mut status_and_r = Self :: result_from_nan (r) ; if b . is_signaling () { status_and_r . status |= Status :: INVALID_OP ; } status_and_r } }
    };
}

impl_43!();