macro_rules! deps {
    () => {
        ExpansionBehavior!();
    };
}

macro_rules! expand_without_refresh {
    () => {
        deps!();
        # [doc = " Attempts to expand macros in files that match glob pattern."] # [doc = " More strict version of [`expand`] function."] # [doc = ""] # [doc = " # Refresh behavior"] # [doc = ""] # [doc = " If no matching `.expanded.rs` files present, it considered a failed test."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Will panic if no matching `.expanded.rs` file is present. Otherwise it will exhibit the same"] # [doc = " behavior as in [`expand`]."] # [doc = ""] # [doc = " [`expand`]: expand/fn.expand.html"] pub fn expand_without_refresh (path : impl AsRef < Path >) { run_tests (path , ExpansionBehavior :: ExpectFiles , Option :: < Vec < String > > :: None ,) ; }
    };
}

expand_without_refresh!();