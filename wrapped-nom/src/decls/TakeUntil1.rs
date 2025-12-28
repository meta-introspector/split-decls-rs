macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! TakeUntil1 {
    () => {
        deps!();
        # [doc = " Parser implementation for take_until1"] pub struct TakeUntil1 < T , E > { tag : T , e : PhantomData < E > , }
    };
}

TakeUntil1!();