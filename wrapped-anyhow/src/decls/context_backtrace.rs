macro_rules! deps {
    () => {
        Ref!();
        ContextError!();
        ErrorImpl!();
        Error!();
        Backtrace!();
    };
}

macro_rules! context_backtrace {
    () => {
        deps!();
        # [cfg (all (not (error_generic_member_access) , any (std_backtrace , feature = "backtrace")))] # [allow (clippy :: unnecessary_wraps)] unsafe fn context_backtrace < C > (e : Ref < ErrorImpl >) -> Option < & Backtrace > where C : 'static , { let unerased_ref = e . cast :: < ErrorImpl < ContextError < C , Error > > > () ; let unerased = unsafe { unerased_ref . deref () } ; let backtrace = unsafe { ErrorImpl :: backtrace (unerased . _object . error . inner . by_ref ()) } ; Some (backtrace) }
    };
}

context_backtrace!();