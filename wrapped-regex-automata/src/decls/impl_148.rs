macro_rules! deps {
    () => {
        Seen!();
        StateID!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl Seen { fn new () -> Seen { Seen { set : alloc :: collections :: BTreeSet :: new () } } fn insert (& mut self , id : StateID) { self . set . insert (id) ; } fn contains (& self , id : & StateID) -> bool { self . set . contains (id) } }
    };
}

impl_148!();