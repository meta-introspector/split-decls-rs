macro_rules! make_output {
    () => {
        # [doc = " Extract output directory and file from matches."] fn make_output (matches : & getopts :: Matches) -> (Option < PathBuf > , Option < OutFileName >) { let odir = matches . opt_str ("out-dir") . map (| o | PathBuf :: from (& o)) ; let ofile = matches . opt_str ("o") . map (| o | match o . as_str () { "-" => OutFileName :: Stdout , path => OutFileName :: Real (PathBuf :: from (path)) , }) ; (odir , ofile) }
    };
}

make_output!();