macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl < A , B > Either < A , B > { # [doc = " Convert `Pin<&Either<A, B>>` to `Either<Pin<&A>, Pin<&B>>`,"] # [doc = " pinned projections of the inner variants."] pub fn as_pin_ref (self : Pin < & Self >) -> Either < Pin < & A > , Pin < & B > > { unsafe { match self . get_ref () { Self :: Left (inner) => Either :: Left (Pin :: new_unchecked (inner)) , Self :: Right (inner) => Either :: Right (Pin :: new_unchecked (inner)) , } } } # [doc = " Convert `Pin<&mut Either<A, B>>` to `Either<Pin<&mut A>, Pin<&mut B>>`,"] # [doc = " pinned projections of the inner variants."] pub fn as_pin_mut (self : Pin < & mut Self >) -> Either < Pin < & mut A > , Pin < & mut B > > { unsafe { match self . get_unchecked_mut () { Self :: Left (inner) => Either :: Left (Pin :: new_unchecked (inner)) , Self :: Right (inner) => Either :: Right (Pin :: new_unchecked (inner)) , } } } }
    };
}

impl_266!();