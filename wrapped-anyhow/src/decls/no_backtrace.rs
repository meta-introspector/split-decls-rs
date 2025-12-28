macro_rules! deps {
    () => {
        Ref!();
        ErrorImpl!();
        Backtrace!();
    };
}

macro_rules! no_backtrace {
    () => {
        deps!();
        # [cfg (all (not (error_generic_member_access) , any (std_backtrace , feature = "backtrace")))] fn no_backtrace (e : Ref < ErrorImpl >) -> Option < & Backtrace > { let _ = e ; None }
    };
}

no_backtrace!()