macro_rules! missing {
    () => {
        # [doc = " Specify that an enum should have no traits that aren't specified in the macro"] # [doc = " invocation, i.e. no `Clone` or `Copy`."] macro_rules ! missing { ($ ($ (# [$ attr : meta]) * pub enum $ i : ident { }) *) => ($ ($ (# [$ attr]) * # [allow (missing_copy_implementations)] pub enum $ i { }) *) ; }
    };
}

missing!();