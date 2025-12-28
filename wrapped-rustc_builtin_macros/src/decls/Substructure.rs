macro_rules! deps {
    () => {
        SubstructureFields!();
    };
}

macro_rules! Substructure {
    () => {
        deps!();
        # [doc = " All the data about the data structure/method being derived upon."] pub (crate) struct Substructure < 'a > { # [doc = " ident of self"] pub type_ident : Ident , # [doc = " Verbatim access to any non-selflike arguments, i.e. arguments that"] # [doc = " don't have type `&Self`."] pub nonselflike_args : & 'a [Box < Expr >] , pub fields : & 'a SubstructureFields < 'a > , }
    };
}

Substructure!()