macro_rules! deps {
    () => {
        CaptureMatches!();
        Captures!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'r , 'h > Iterator for CaptureMatches < 'r , 'h > { type Item = Captures < 'h > ; # [inline] fn next (& mut self) -> Option < Captures < 'h > > { let static_captures_len = self . it . regex () . static_captures_len () ; self . it . next () . map (| caps | Captures { haystack : self . haystack , caps , static_captures_len , }) } # [inline] fn count (self) -> usize { self . it . count () } }
    };
}

impl_48!();