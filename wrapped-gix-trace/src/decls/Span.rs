macro_rules! Span {
    () => {
        # [doc = " A workaround for a clippy warning"] # [doc (hidden)] # [derive (Clone)] pub struct Span ;
    };
}

Span!();