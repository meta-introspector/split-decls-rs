macro_rules! deps {
    () => {
        HeadTail!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < I > HeadTail < I > where I : Iterator , { # [doc = " Constructs a `HeadTail` from an `Iterator`. Returns `None` if the `Iterator` is empty."] fn new (mut it : I) -> Option < Self > { let head = it . next () ; head . map (| h | Self { head : h , tail : it }) } # [doc = " Get the next element and update `head`, returning the old head in `Some`."] # [doc = ""] # [doc = " Returns `None` when the tail is exhausted (only `head` then remains)."] fn next (& mut self) -> Option < I :: Item > { if let Some (next) = self . tail . next () { Some (replace (& mut self . head , next)) } else { None } } # [doc = " Hints at the size of the sequence, same as the `Iterator` method."] fn size_hint (& self) -> (usize , Option < usize >) { size_hint :: add_scalar (self . tail . size_hint () , 1) } }
    };
}

impl_322!();