macro_rules! deps {
    () => {
        Needed!();
        Err!();
    };
}

macro_rules! IsStreaming {
    () => {
        deps!();
        # [doc = " Specifies the behaviour when a parser encounters an error that could be due to partial ata"] pub trait IsStreaming { # [doc = " called by parsers on partial data errors"] # [doc = " * `needed` can hold the amount of additional data the parser would need to decide"] # [doc = " * `err_f`: produces the error when in \"complete\" mode"] fn incomplete < E , F : FnOnce () -> E > (needed : Needed , err_f : F) -> Err < E > ; # [doc = " Indicates whether the data is in streaming mode or not"] fn is_streaming () -> bool ; }
    };
}

IsStreaming!();