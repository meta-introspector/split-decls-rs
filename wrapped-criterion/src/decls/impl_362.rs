macro_rules! deps {
    () => {
        Label!();
        Iter!();
        Float!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        impl < 'a , A > Iterator for Iter < 'a , A > where A : Float , { type Item = (A , Label) ; # [allow (clippy :: similar_names)] fn next (& mut self) -> Option < (A , Label) > { self . iter . next () . map (| & x | { let (lost , lomt , himt , hist) = self . fences ; let label = if x < lost { LowSevere } else if x > hist { HighSevere } else if x < lomt { LowMild } else if x > himt { HighMild } else { NotAnOutlier } ; (x , label) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_362!();