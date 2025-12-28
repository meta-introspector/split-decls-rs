macro_rules! deps {
    () => {
        Reader!();
        UnitIndexSectionIterator!();
        UnitIndexSection!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        impl < 'index , R : Reader > Iterator for UnitIndexSectionIterator < 'index , R > { type Item = UnitIndexSection ; fn next (& mut self) -> Option < UnitIndexSection > { let section = * self . sections . next () ? ; let offset = self . offsets . read_u32 () . ok () ? ; let size = self . sizes . read_u32 () . ok () ? ; Some (UnitIndexSection { section , offset , size , }) } }
    };
}

impl_393!();