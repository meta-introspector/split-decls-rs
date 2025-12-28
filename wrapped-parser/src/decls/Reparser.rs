macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Reparser {
    () => {
        deps!();
        # [doc = " A parsing function for a specific braced-block."] pub struct Reparser (fn (& mut parser :: Parser < '_ >)) ;
    };
}

Reparser!();