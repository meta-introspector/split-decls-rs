macro_rules! deps {
    () => {
        Seen!();
        StateID!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        # [cfg (not (feature = "alloc"))] impl Seen { fn new () -> Seen { Seen { set : core :: marker :: PhantomData } } fn insert (& mut self , _id : StateID) { } fn contains (& self , _id : & StateID) -> bool { true } }
    };
}

impl_149!();