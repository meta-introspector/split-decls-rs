macro_rules! deps {
    () => {
        WordIndices!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < 'a > Iterator for WordIndices < 'a > { type Item = (usize , usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < (usize , usize , & 'a str) > { for (start , end , word) in self . 0 . by_ref () { let input = Input :: new (word) . anchored (Anchored :: Yes) . earliest (true) ; if SIMPLE_WORD_FWD . try_search_fwd (& input) . unwrap () . is_some () { return Some ((start , end , word)) ; } } None } }
    };
}

impl_206!();