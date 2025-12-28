macro_rules! deps {
    () => {
        Bag!();
        AtomicEpoch!();
        Entry!();
        Collector!();
    };
}

macro_rules! Local {
    () => {
        deps!();
        # [doc = " Participant for garbage collection."] # [repr (C)] pub (crate) struct Local { # [doc = " A node in the intrusive linked list of `Local`s."] entry : Entry , # [doc = " A reference to the global data."] # [doc = ""] # [doc = " When all guards and handles get dropped, this reference is destroyed."] collector : UnsafeCell < ManuallyDrop < Collector > > , # [doc = " The local bag of deferred functions."] pub (crate) bag : UnsafeCell < Bag > , # [doc = " The number of guards keeping this participant pinned."] guard_count : Cell < usize > , # [doc = " The number of active handles."] handle_count : Cell < usize > , # [doc = " Total number of pinnings performed."] # [doc = ""] # [doc = " This is just an auxiliary counter that sometimes kicks off collection."] pin_count : Cell < Wrapping < usize > > , # [doc = " The local epoch."] epoch : CachePadded < AtomicEpoch > , }
    };
}

Local!();