macro_rules! deps {
    () => {
        ThreadData!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl ThreadData { # [inline] fn new () -> ThreadData { assert ! (mem :: align_of ::< ThreadData > () > ! QUEUE_MASK) ; ThreadData { parker : ThreadParker :: new () , queue_tail : Cell :: new (ptr :: null ()) , prev : Cell :: new (ptr :: null ()) , next : Cell :: new (ptr :: null ()) , } } }
    };
}

impl_55!();