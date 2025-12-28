macro_rules! impl_579 {
    () => {
        impl < St : Stream + UnwindSafe > CatchUnwind < St > { pub (super) fn new (stream : St) -> Self { Self { stream , caught_unwind : false } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_579!()