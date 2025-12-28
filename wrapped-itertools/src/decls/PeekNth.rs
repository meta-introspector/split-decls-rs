macro_rules! PeekNth {
    () => {
        # [doc = " See [`peek_nth()`] for more information."] # [derive (Clone , Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct PeekNth < I > where I : Iterator , { iter : Fuse < I > , buf : VecDeque < I :: Item > , }
    };
}

PeekNth!()