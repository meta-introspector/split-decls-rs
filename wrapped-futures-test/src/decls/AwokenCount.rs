macro_rules! deps {
    () => {
        WakerInner!();
    };
}

macro_rules! AwokenCount {
    () => {
        deps!();
        # [doc = " Number of times the waker was awoken."] # [doc = ""] # [doc = " See [`new_count_waker`] for usage."] # [derive (Debug)] pub struct AwokenCount { inner : Arc < WakerInner > , }
    };
}

AwokenCount!()