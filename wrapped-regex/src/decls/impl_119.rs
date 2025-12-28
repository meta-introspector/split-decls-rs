macro_rules! deps {
    () => {
        Match!();
        SubCaptureMatches!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'c , 'h > Iterator for SubCaptureMatches < 'c , 'h > { type Item = Option < Match < 'h > > ; # [inline] fn next (& mut self) -> Option < Option < Match < 'h > > > { self . it . next () . map (| group | { group . map (| sp | Match :: new (self . haystack , sp . start , sp . end)) }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } # [inline] fn count (self) -> usize { self . it . count () } }
    };
}

impl_119!()