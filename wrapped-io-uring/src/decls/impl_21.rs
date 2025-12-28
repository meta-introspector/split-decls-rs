macro_rules! deps {
    () => {
        EntryMarker!();
        CompletionQueue!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < E : EntryMarker > ExactSizeIterator for CompletionQueue < '_ , E > { # [inline] fn len (& self) -> usize { self . tail . wrapping_sub (self . head) as usize } }
    };
}

impl_21!();