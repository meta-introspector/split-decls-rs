macro_rules! proc_macro_span_file {
    () => {
        # [cfg (proc_macro_span_file)] pub (crate) mod proc_macro_span_file ;
    };
}

proc_macro_span_file!()