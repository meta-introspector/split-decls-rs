macro_rules! hide_from_rustfmt {
    () => {
        # [allow (unused_macros)] macro_rules ! hide_from_rustfmt { ($ mod : item) => { $ mod } ; }
    };
}

hide_from_rustfmt!();