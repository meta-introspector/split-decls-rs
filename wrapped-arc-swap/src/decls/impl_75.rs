macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Default for Node { fn default () -> Self { Node { fast : FastSlots :: default () , helping : HelpingSlots :: default () , in_use : AtomicUsize :: new (NODE_USED) , next : ptr :: null () , active_writers : AtomicUsize :: new (0) , } } }
    };
}

impl_75!();