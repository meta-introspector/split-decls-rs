macro_rules! Reparser {
    () => {
        # [doc = " A parsing function for a specific braced-block."] pub struct Reparser (fn (& mut parser :: Parser < '_ >)) ;
    };
}

Reparser!()