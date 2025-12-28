macro_rules! deps {
    () => {
        PeekingNext!();
        RepeatN!();
    };
}

macro_rules! impl_408 {
    () => {
        deps!();
        impl < T : Clone > PeekingNext for RepeatN < T > { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { let r = self . elt . as_ref () ? ; if ! accept (r) { return None ; } self . next () } }
    };
}

impl_408!();