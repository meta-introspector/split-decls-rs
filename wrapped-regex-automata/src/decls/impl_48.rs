macro_rules! deps {
    () => {
        StateIter!();
        State!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a , T : AsRef < [u32] > > Iterator for StateIter < 'a , T > { type Item = State < 'a > ; fn next (& mut self) -> Option < State < 'a > > { self . it . next () . map (| (index , _) | { let id = self . tt . to_state_id (index) ; self . tt . state (id) }) } }
    };
}

impl_48!();