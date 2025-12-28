macro_rules! deps {
    () => {
        IIterator!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType > Iterator for IIterator < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { let result = if self . HasCurrent () . unwrap_or (false) { self . Current () . ok () } else { None } ; if result . is_some () { self . MoveNext () . ok () ? ; } result } }
    };
}

impl_22!();