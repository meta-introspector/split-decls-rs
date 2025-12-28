macro_rules! deps {
    () => {
        ParseResultBase!();
        ErrParse!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < T > ParseResultBase < T > for ErrParse < T > { fn handle_failure (& mut self , tracker : & mut DynMTrackerTrait ! ()) { for failure in & self . failures { tracker . build_failure (rustc_ast :: token :: Token :: dummy () , failure . lo . lo () . 0 , "macro parsing failure" ,) ; } } fn is_ok (& self) -> bool { false } fn unwrap (self) -> Option < T > { None } }
    };
}

impl_23!();