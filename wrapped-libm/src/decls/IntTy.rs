macro_rules! deps {
    () => {
        Float!();
        Int!();
    };
}

macro_rules! IntTy {
    () => {
        deps!();
        # [doc = " Access the associated `Int` type from a float (helper to avoid ambiguous associated types)."] pub type IntTy < F > = < F as Float > :: Int ;
    };
}

IntTy!();