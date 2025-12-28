macro_rules! TickToken {
    () => {
        # [doc = " Result of a receive operation."] pub (crate) type TickToken = Option < Instant > ;
    };
}

TickToken!();