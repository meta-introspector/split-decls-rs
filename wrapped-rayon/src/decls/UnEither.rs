macro_rules! deps {
    () => {
        UnzipOp!();
    };
}

macro_rules! UnEither {
    () => {
        deps!();
        # [doc = " An `UnzipOp` that routes items depending on their `Either` variant."] struct UnEither ;
    };
}

UnEither!()