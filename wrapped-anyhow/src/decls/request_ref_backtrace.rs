macro_rules! deps {
    () => {
        Error!();
        Backtrace!();
    };
}

macro_rules! request_ref_backtrace {
    () => {
        deps!();
        pub fn request_ref_backtrace (err : & dyn Error) -> Option < & Backtrace > { request_ref :: < Backtrace > (err) }
    };
}

request_ref_backtrace!()