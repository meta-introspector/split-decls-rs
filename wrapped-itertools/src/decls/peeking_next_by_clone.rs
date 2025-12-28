macro_rules! deps {
    () => {
        PeekingNext!();
    };
}

macro_rules! peeking_next_by_clone {
    () => {
        deps!();
        macro_rules ! peeking_next_by_clone { ([$ ($ typarm : tt) *] $ type_ : ty) => { impl <$ ($ typarm) *> PeekingNext for $ type_ { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool { let saved_state = self . clone () ; if let Some (r) = self . next () { if ! accept (& r) { * self = saved_state ; } else { return Some (r) } } None } } } }
    };
}

peeking_next_by_clone!();