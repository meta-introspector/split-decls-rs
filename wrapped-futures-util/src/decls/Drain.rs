macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! Drain {
    () => {
        deps!();
        # [doc = " Sink for the [`drain`] function."] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] pub struct Drain < T > { marker : PhantomData < T > , }
    };
}

Drain!()