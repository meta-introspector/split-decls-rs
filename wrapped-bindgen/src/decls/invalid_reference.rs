macro_rules! invalid_reference {
    () => {
        # [track_caller] fn invalid_reference () -> ! { panic ! ("`--reference` must be `<crate>,<full/flat/skip-root>,<type name>") ; }
    };
}

invalid_reference!();