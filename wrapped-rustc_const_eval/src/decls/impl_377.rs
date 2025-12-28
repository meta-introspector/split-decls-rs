macro_rules! deps {
    () => {
        TypeNamePrinter!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl < 'tcx > PrettyPrinter < 'tcx > for TypeNamePrinter < 'tcx > { fn should_print_optional_region (& self , region : ty :: Region < '_ >) -> bool { let kind = region . kind () ; match region . kind () { ty :: ReErased | ty :: ReEarlyParam (_) | ty :: ReStatic => false , ty :: ReBound (..) => true , _ => panic ! ("type_name unhandled region: {kind:?}") , } } fn generic_delimiters (& mut self , f : impl FnOnce (& mut Self) -> Result < () , PrintError > ,) -> Result < () , PrintError > { write ! (self , "<") ? ; f (self) ? ; write ! (self , ">") ? ; Ok (()) } fn should_print_verbose (& self) -> bool { false } }
    };
}

impl_377!()