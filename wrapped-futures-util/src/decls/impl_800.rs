macro_rules! deps {
    () => {
        PollNext!();
        InternalState!();
    };
}

macro_rules! impl_800 {
    () => {
        deps!();
        impl InternalState { fn finish (& mut self , ps : PollNext) { match (& self , ps) { (Self :: Start , PollNext :: Left) => { * self = Self :: LeftFinished ; } (Self :: Start , PollNext :: Right) => { * self = Self :: RightFinished ; } (Self :: LeftFinished , PollNext :: Right) | (Self :: RightFinished , PollNext :: Left) => { * self = Self :: BothFinished ; } _ => { } } } }
    };
}

impl_800!()