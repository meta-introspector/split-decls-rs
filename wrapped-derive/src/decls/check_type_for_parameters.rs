macro_rules! deps {
    () => {
        TypeVisitor!();
    };
}

macro_rules! check_type_for_parameters {
    () => {
        deps!();
        # [doc = " Checks if a type has type or lifetime parameters, given the local context of"] # [doc = " named type parameters. Returns (has_type_params, has_lifetime_params)"] pub fn check_type_for_parameters (ty : & Type , typarams : & HashMap < Ident , Option < Ident > > ,) -> (bool , bool) { let mut visit = TypeVisitor { typarams , found_typarams : false , found_lifetimes : false , } ; visit_type (& mut visit , ty) ; (visit . found_typarams , visit . found_lifetimes) }
    };
}

check_type_for_parameters!()