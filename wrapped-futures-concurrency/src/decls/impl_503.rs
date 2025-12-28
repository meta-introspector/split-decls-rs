macro_rules! deps {
    () => {
        Zip!();
    };
}

macro_rules! impl_503 {
    () => {
        deps!();
        # [doc = " Drop the already initialized values on cancellation."] # [pinned_drop] impl < S > PinnedDrop for Zip < S > where S : Stream , { fn drop (self : Pin < & mut Self >) { let this = self . project () ; for (state , output) in this . state . iter_mut () . zip (this . output . iter_mut ()) { if state . is_ready () { unsafe { output . assume_init_drop () } ; } } } }
    };
}

impl_503!()