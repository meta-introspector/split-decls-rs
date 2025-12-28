macro_rules! deps {
    () => {
        CfiEntriesIter!();
        Error!();
        UnwindSection!();
        Section!();
        CieOrFde!();
        Result!();
        Reader!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < 'bases , Section , R > fallible_iterator :: FallibleIterator for CfiEntriesIter < 'bases , Section , R > where R : Reader , Section : UnwindSection < R > , { type Item = CieOrFde < 'bases , Section , R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { CfiEntriesIter :: next (self) } }
    };
}

impl_195!();