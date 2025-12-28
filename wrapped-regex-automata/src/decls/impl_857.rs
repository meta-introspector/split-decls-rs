macro_rules! deps {
    () => {
        StateBuilderMatches!();
        Repr!();
        StateBuilderEmpty!();
        ReprVec!();
    };
}

macro_rules! impl_857 {
    () => {
        deps!();
        # [doc = " For docs on these routines, see the internal Repr and ReprVec types below."] impl StateBuilderEmpty { pub (crate) fn new () -> StateBuilderEmpty { StateBuilderEmpty (alloc :: vec ! []) } pub (crate) fn into_matches (mut self) -> StateBuilderMatches { self . 0 . extend_from_slice (& [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0]) ; StateBuilderMatches (self . 0) } fn clear (& mut self) { self . 0 . clear () ; } pub (crate) fn capacity (& self) -> usize { self . 0 . capacity () } }
    };
}

impl_857!();