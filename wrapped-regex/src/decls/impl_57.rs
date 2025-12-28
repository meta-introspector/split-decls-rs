macro_rules! deps {
    () => {
        CaptureNames!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'r > Iterator for CaptureNames < 'r > { type Item = Option < & 'r str > ; # [inline] fn next (& mut self) -> Option < Option < & 'r str > > { self . 0 . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } # [inline] fn count (self) -> usize { self . 0 . count () } }
    };
}

impl_57!();