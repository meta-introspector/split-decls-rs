macro_rules! _bail {
    () => {
        # [macro_export] macro_rules ! _bail { ($ ($ tokens : tt) *) => { return Err (format_err ! ($ ($ tokens) *)) } }
    };
}

_bail!()