macro_rules! cpp_like_debuginfo {
    () => {
        # [doc = " Check if we should generate C++ like names and debug information."] pub fn cpp_like_debuginfo (tcx : TyCtxt < '_ >) -> bool { tcx . sess . target . is_like_msvc }
    };
}

cpp_like_debuginfo!();