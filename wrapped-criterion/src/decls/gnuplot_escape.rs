macro_rules! gnuplot_escape {
    () => {
        fn gnuplot_escape (string : & str) -> String { string . replace ('_' , "\\_") . replace ('\'' , "''") }
    };
}

gnuplot_escape!();