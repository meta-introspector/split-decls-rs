macro_rules! deps {
    () => {
        OwnedStorage!();
        QueueInner!();
    };
}

macro_rules! Queue {
    () => {
        deps!();
        # [doc = " A statically allocated single-producer, single-consumer queue with a capacity of `N - 1`"] # [doc = " elements."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " To get better performance, use a value for `N` that is a power of 2."] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " You will likely want to use [`split`](QueueInner::split) to create a producer-consumer pair."] pub type Queue < T , const N : usize > = QueueInner < T , OwnedStorage < N > > ;
    };
}

Queue!()