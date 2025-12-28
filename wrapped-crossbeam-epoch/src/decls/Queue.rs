macro_rules! deps {
    () => {
        Atomic!();
        Node!();
    };
}

macro_rules! Queue {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Queue < T > { head : CachePadded < Atomic < Node < T > > > , tail : CachePadded < Atomic < Node < T > > > , }
    };
}

Queue!()