macro_rules! debug_reprs {
    () => {
        # [doc = " Return a vector of the debug formatting of each item in `args`"] fn debug_reprs (args : & [& dyn Debug]) -> Vec < String > { args . iter () . map (| x | format ! ("{x:?}")) . collect () }
    };
}

debug_reprs!();