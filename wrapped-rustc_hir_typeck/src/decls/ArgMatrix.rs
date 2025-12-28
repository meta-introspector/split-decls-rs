macro_rules! deps {
    () => {
        Compatibility!();
    };
}

macro_rules! ArgMatrix {
    () => {
        deps!();
        pub (crate) struct ArgMatrix < 'tcx > { # [doc = " Maps the indices in the `compatibility_matrix` rows to the indices of"] # [doc = " the *user provided* inputs"] provided_indices : Vec < ProvidedIdx > , # [doc = " Maps the indices in the `compatibility_matrix` columns to the indices"] # [doc = " of the *expected* args"] expected_indices : Vec < ExpectedIdx > , # [doc = " The first dimension (rows) are the remaining user provided inputs to"] # [doc = " match and the second dimension (cols) are the remaining expected args"] # [doc = " to match"] compatibility_matrix : Vec < Vec < Compatibility < 'tcx > > > , }
    };
}

ArgMatrix!()