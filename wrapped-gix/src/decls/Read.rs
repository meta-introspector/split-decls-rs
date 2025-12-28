macro_rules! Read {
    () => {
        # [doc = " A wrapper for implementers of [`std::io::Read`] or [`std::io::BufRead`] with interrupt support."] # [doc = ""] # [doc = " It fails a [read][`std::io::Read::read`] while an interrupt was requested."] pub struct Read < R > { # [doc = " The actual implementor of [`std::io::Read`] to which interrupt support will be added."] inner : gix_features :: interrupt :: Read < 'static , R > , }
    };
}

Read!();