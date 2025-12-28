macro_rules! CharAccumulator {
    () => {
        # [doc = " Build a `char` out of bytes"] pub trait CharAccumulator : Default { # [doc = " Build a `char` out of bytes"] # [doc = ""] # [doc = " Return `None` when more data is needed"] fn add (& mut self , byte : u8) -> Option < char > ; }
    };
}

CharAccumulator!();