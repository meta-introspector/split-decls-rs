macro_rules! print_wall_help {
    () => {
        fn print_wall_help () { safe_println ! ("
The flag `-Wall` does not exist in `rustc`. Most useful lints are enabled by
default. Use `rustc -W help` to see all available lints. It's more common to put
warning settings in the crate root using `#![warn(LINT_NAME)]` instead of using
the command line flag directly.
") ; }
    };
}

print_wall_help!();