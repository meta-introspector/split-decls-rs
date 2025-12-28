macro_rules! dimensions {
    () => {
        pub (crate) fn dimensions () -> (Option < usize > , Option < usize >) { # [cfg (not (feature = "wrap_help"))] return (None , None) ; # [cfg (feature = "wrap_help")] terminal_size :: terminal_size () . map (| (w , h) | (Some (w . 0 . into ()) , Some (h . 0 . into ()))) . unwrap_or_else (| | (parse_env ("COLUMNS") , parse_env ("LINES"))) }
    };
}

dimensions!();