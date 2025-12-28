macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! Position {
    () => {
        deps!();
        # [doc = " A position in a queue."] struct Position < T > { # [doc = " The index in the queue."] index : AtomicUsize , # [doc = " The block in the linked list."] block : AtomicPtr < Block < T > > , }
    };
}

Position!()