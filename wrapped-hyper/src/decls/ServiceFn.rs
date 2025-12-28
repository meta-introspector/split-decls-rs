macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! ServiceFn {
    () => {
        deps!();
        # [doc = " Service returned by [`service_fn`]"] pub struct ServiceFn < F , R > { f : F , _req : PhantomData < fn (R) > , }
    };
}

ServiceFn!();