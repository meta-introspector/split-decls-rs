macro_rules! fail_match {
    () => {
        macro_rules ! fail_match { ($ ($ args : tt) *) => { return Err (match_error ! ($ ($ args) *)) } ; }
    };
}

fail_match!();