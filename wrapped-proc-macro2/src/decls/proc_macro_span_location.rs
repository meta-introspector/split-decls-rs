macro_rules! proc_macro_span_location {
    () => {
        # [cfg (proc_macro_span_location)] pub (crate) mod proc_macro_span_location ;
    };
}

proc_macro_span_location!();