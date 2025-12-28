macro_rules! deps {
    () => {
        ParseResultBase!();
        OkParse!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T > ParseResultBase < T > for OkParse < T > { fn handle_failure (& mut self , _tracker : & mut DynMTrackerTrait ! ()) { } fn is_ok (& self) -> bool { true } fn unwrap (self) -> Option < T > { Some (self . 0) } }
    };
}

impl_17!()