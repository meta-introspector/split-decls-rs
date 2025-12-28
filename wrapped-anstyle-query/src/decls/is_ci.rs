macro_rules! is_ci {
    () => {
        # [doc = " Report whether this is running in CI"] # [doc = ""] # [doc = " CI is a common environment where, despite being piped, ansi color codes are supported"] # [doc = ""] # [doc = " This is not as exhaustive as you'd find in a crate like `is_ci` but it should work in enough"] # [doc = " cases."] # [inline] pub fn is_ci () -> bool { std :: env :: var_os ("CI") . is_some () }
    };
}

is_ci!()