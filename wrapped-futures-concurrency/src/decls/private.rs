macro_rules! private {
    () => {
        # [doc (hidden)] pub mod private ;
    };
}

private!();