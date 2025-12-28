macro_rules! deps {
    () => {
        NextAction!();
        Context!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl From < Context > for NextAction { fn from (ctx : Context) -> Self { let mut buf = Vec :: < u8 > :: new () ; ctx . write_to (& mut buf) . expect ("cannot fail") ; NextAction { previous_output : buf . into () , } } }
    };
}

impl_11!();