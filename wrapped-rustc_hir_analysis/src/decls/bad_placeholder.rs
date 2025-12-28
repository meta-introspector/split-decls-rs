macro_rules! deps {
    () => {
        HirTyLowerer!();
        PlaceholderNotAllowedItemSignatures!();
    };
}

macro_rules! bad_placeholder {
    () => {
        deps!();
        fn bad_placeholder < 'cx , 'tcx > (cx : & 'cx dyn HirTyLowerer < 'tcx > , mut spans : Vec < Span > , kind : & 'static str ,) -> Diag < 'cx > { let kind = if kind . ends_with ('s') { format ! ("{kind}es") } else { format ! ("{kind}s") } ; spans . sort () ; cx . dcx () . create_err (errors :: PlaceholderNotAllowedItemSignatures { spans , kind }) }
    };
}

bad_placeholder!()