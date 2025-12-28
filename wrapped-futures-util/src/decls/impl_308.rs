macro_rules! impl_308 {
    () => {
        impl < St > Cycle < St > where St : Clone + Stream , { pub (super) fn new (stream : St) -> Self { Self { orig : stream . clone () , stream } } }
    };
}

impl_308!();