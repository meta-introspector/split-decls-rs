macro_rules! deps {
    () => {
        Flags!();
        IterDefinedNames!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < B : Flags > Iterator for IterDefinedNames < B > { type Item = (& 'static str , B) ; fn next (& mut self) -> Option < Self :: Item > { while let Some (flag) = self . flags . get (self . idx) { self . idx += 1 ; if flag . is_named () { return Some ((flag . name () , B :: from_bits_retain (flag . value () . bits ()))) ; } } None } }
    };
}

impl_110!()