macro_rules! SizeHint {
    () => {
        # [doc = " `SizeHint` is the return type of `Iterator::size_hint()`."] pub type SizeHint = (usize , Option < usize >) ;
    };
}

SizeHint!()