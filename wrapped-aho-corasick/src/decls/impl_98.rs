macro_rules! deps {
    () => {
        StateID!();
        QueuedSet!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl QueuedSet { # [doc = " Return an inert set that returns `false` for every state ID membership"] # [doc = " test."] fn inert () -> QueuedSet { QueuedSet { set : None } } # [doc = " Return an active set that tracks state ID membership."] fn active () -> QueuedSet { QueuedSet { set : Some (BTreeSet :: new ()) } } # [doc = " Inserts the given state ID into this set. (If the set is inert, then"] # [doc = " this is a no-op.)"] fn insert (& mut self , state_id : StateID) { if let Some (ref mut set) = self . set { set . insert (state_id) ; } } # [doc = " Returns true if and only if the given state ID is in this set. If the"] # [doc = " set is inert, this always returns false."] fn contains (& self , state_id : StateID) -> bool { match self . set { None => false , Some (ref set) => set . contains (& state_id) , } } }
    };
}

impl_98!()