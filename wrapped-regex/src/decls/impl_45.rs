macro_rules! deps {
    () => {
        Matches!();
        Match!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < 'r , 'h > Iterator for Matches < 'r , 'h > { type Item = Match < 'h > ; # [inline] fn next (& mut self) -> Option < Match < 'h > > { self . it . next () . map (| sp | Match :: new (self . haystack , sp . start () , sp . end ())) } # [inline] fn count (self) -> usize { self . it . count () } }
    };
}

impl_45!();