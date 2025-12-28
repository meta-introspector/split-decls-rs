macro_rules! deps {
    () => {
        Backtrace!();
    };
}

macro_rules! provide_ref_backtrace {
    () => {
        deps!();
        pub fn provide_ref_backtrace < 'a > (request : & mut Request < 'a > , backtrace : & 'a Backtrace) { Request :: provide_ref (request , backtrace) ; }
    };
}

provide_ref_backtrace!()