macro_rules! deps {
    () => {
        CompletionQueue!();
        EntryMarker!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < E : EntryMarker > Iterator for CompletionQueue < '_ , E > { type Item = E ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . head != self . tail { Some (unsafe { self . pop () }) } else { None } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
    };
}

impl_20!()