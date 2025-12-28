macro_rules! _bail {
    () => {
        # [doc = " Returns from the current function with an error, supplied by arguments as for format!"] macro_rules ! _bail { ($ ($ tokens : tt) *) => { return Err (crate :: errors :: error ! ($ ($ tokens) *)) } }
    };
}

_bail!();