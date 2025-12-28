macro_rules! deps {
    () => {
        Join!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        # [doc = " Drop the already initialized values on cancellation."] # [pinned_drop] impl < Fut , const N : usize > PinnedDrop for Join < Fut , N > where Fut : Future , { fn drop (self : Pin < & mut Self >) { let mut this = self . project () ; for i in this . state . ready_indexes () { unsafe { this . items . drop (i) } ; } for i in this . state . pending_indexes () { unsafe { this . futures . as_mut () . drop (i) } ; } } }
    };
}

impl_222!()