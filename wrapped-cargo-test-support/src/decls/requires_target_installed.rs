macro_rules! requires_target_installed {
    () => {
        # [doc = " Check if the given target has been installed."] # [doc = ""] # [doc = " Generally `testsuite::utils::cross_compile::disabled` should be used to check if cross-compilation is allowed."] # [doc = " And [`alternate`] to get the cross target."] # [doc = ""] # [doc = " You should only use this as a last resort to skip tests,"] # [doc = " because it doesn't report skipped tests as ignored."] pub fn requires_target_installed (target : & str) -> bool { let has_target = std :: process :: Command :: new ("rustup") . args (["target" , "list" , "--installed"]) . output () . ok () . map (| output | { String :: from_utf8 (output . stdout) . map (| stdout | stdout . contains (target)) . unwrap_or_default () }) . unwrap_or_default () ; if ! has_target { let msg = format ! ("to run this test, run `rustup target add {target} --toolchain <toolchain>`" ,) ; if cargo_util :: is_ci () { panic ! ("{msg}") ; } else { eprintln ! ("{msg}") ; } } has_target }
    };
}

requires_target_installed!();