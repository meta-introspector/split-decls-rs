macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! Queue {
    () => {
        deps!();
        # [doc = " The multi-producer single-consumer structure. This is not cloneable, but it"] # [doc = " may be safely shared so long as it is guaranteed that there is only one"] # [doc = " popper at a time (many pushers are allowed)."] pub (super) struct Queue < T > { head : AtomicPtr < Node < T > > , tail : UnsafeCell < * mut Node < T > > , }
    };
}

Queue!()