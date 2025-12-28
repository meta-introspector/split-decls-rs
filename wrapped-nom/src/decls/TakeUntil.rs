macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! TakeUntil {
    () => {
        deps!();
        # [doc = " Parser implementation for [take_until]"] pub struct TakeUntil < T , E > { tag : T , e : PhantomData < E > , }
    };
}

TakeUntil!();