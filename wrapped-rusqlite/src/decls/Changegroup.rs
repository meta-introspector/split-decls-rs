macro_rules! Changegroup {
    () => {
        # [doc = " Used to combine two or more changesets or"] # [doc = " patchsets"] pub struct Changegroup { cg : * mut ffi :: sqlite3_changegroup , }
    };
}

Changegroup!();