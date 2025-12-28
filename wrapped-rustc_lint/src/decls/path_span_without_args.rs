macro_rules! path_span_without_args {
    () => {
        # [doc = " Return for a given `Path` the span until the last args"] fn path_span_without_args (path : & Path < '_ >) -> Span { if let Some (args) = & path . segments . last () . unwrap () . args { path . span . until (args . span_ext) } else { path . span } }
    };
}

path_span_without_args!();