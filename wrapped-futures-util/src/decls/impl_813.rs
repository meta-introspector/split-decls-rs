macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_813 {
    () => {
        deps!();
        impl < T , F , Fut , Item > FusedStream for Unfold < T , F , Fut > where F : FnMut (T) -> Fut , Fut : Future < Output = Option < (Item , T) > > , { fn is_terminated (& self) -> bool { matches ! (self . state , UnfoldState :: Empty) } }
    };
}

impl_813!();