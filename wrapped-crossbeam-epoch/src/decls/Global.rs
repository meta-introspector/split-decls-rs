macro_rules! deps {
    () => {
        List!();
        Local!();
        SealedBag!();
        AtomicEpoch!();
        Queue!();
    };
}

macro_rules! Global {
    () => {
        deps!();
        # [doc = " The global data for a garbage collector."] pub (crate) struct Global { # [doc = " The intrusive linked list of `Local`s."] locals : List < Local > , # [doc = " The global queue of bags of deferred functions."] queue : Queue < SealedBag > , # [doc = " The global epoch."] pub (crate) epoch : CachePadded < AtomicEpoch > , }
    };
}

Global!();