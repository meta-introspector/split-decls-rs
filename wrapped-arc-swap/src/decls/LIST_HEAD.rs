macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! LIST_HEAD {
    () => {
        deps!();
        # [doc = " The head of the debt linked list."] static LIST_HEAD : AtomicPtr < Node > = AtomicPtr :: new (ptr :: null_mut ()) ;
    };
}

LIST_HEAD!();