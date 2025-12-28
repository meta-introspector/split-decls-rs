macro_rules! deps {
    () => {
        InternalState!();
        PollNext!();
    };
}

macro_rules! impl_804 {
    () => {
        deps!();
        impl < St1 , St2 , Clos , State > FusedStream for SelectWithStrategy < St1 , St2 , Clos , State > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , Clos : FnMut (& mut State) -> PollNext , { fn is_terminated (& self) -> bool { matches ! (self . internal_state , InternalState :: BothFinished) } }
    };
}

impl_804!();