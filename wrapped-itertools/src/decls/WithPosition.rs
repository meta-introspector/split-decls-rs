macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! WithPosition {
    () => {
        deps!();
        # [doc = " An iterator adaptor that wraps each element in an [`Position`]."] # [doc = ""] # [doc = " Iterator element type is `(Position, I::Item)`."] # [doc = ""] # [doc = " See [`.with_position()`](crate::Itertools::with_position) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct WithPosition < I > where I : Iterator , { handled_first : bool , peekable : Peekable < Fuse < I > > , }
    };
}

WithPosition!()