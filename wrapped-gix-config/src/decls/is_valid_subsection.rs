macro_rules! is_valid_subsection {
    () => {
        # [doc = " Return true if `name` is valid as subsection name, like `origin` in `[remote \"origin\"]`."] pub fn is_valid_subsection (name : & BStr) -> bool { name . find_byteset (b"\n\0") . is_none () }
    };
}

is_valid_subsection!()