macro_rules! proc_macro_span {
    () => {
        # [cfg (proc_macro_span)] pub (crate) mod proc_macro_span ;
    };
}

proc_macro_span!()