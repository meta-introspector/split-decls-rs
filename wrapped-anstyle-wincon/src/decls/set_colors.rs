macro_rules! set_colors {
    () => {
        # [doc = " Apply colors to future writes"] # [doc = ""] # [doc = " **Note:** Make sure any buffers are first flushed or else these colors will apply"] pub fn set_colors < S : AsHandle > (stream : & mut S , fg : anstyle :: AnsiColor , bg : anstyle :: AnsiColor ,) -> std :: io :: Result < () > { set_colors_ (stream , fg , bg) . map_err (Into :: into) }
    };
}

set_colors!();