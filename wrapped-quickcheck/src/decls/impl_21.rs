macro_rules! deps {
    () => {
        VecShrinker!();
        Arbitrary!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < A > Iterator for VecShrinker < A > where A : Arbitrary , { type Item = Vec < A > ; fn next (& mut self) -> Option < Vec < A > > { if self . size == self . seed . len () { self . size /= 2 ; self . offset = self . size ; return Some (vec ! []) ; } if self . size != 0 { let xs1 = self . seed [.. (self . offset - self . size)] . iter () . chain (& self . seed [self . offset ..]) . cloned () . collect () ; self . offset += self . size ; if self . offset > self . seed . len () { self . size /= 2 ; self . offset = self . size ; } Some (xs1) } else { if self . offset == 0 { self . offset = 1 ; } match self . next_element () { Some (e) => Some (self . seed [.. self . offset - 1] . iter () . cloned () . chain (Some (e)) . chain (self . seed [self . offset ..] . iter () . cloned ()) . collect () ,) , None => None , } } } }
    };
}

impl_21!()