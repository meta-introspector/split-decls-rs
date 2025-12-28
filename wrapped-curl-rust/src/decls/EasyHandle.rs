macro_rules! deps {
    () => {
        DetachGuard!();
        Multi!();
        Easy!();
    };
}

macro_rules! EasyHandle {
    () => {
        deps!();
        # [doc = " Wrapper around an easy handle while it's owned by a multi handle."] # [doc = ""] # [doc = " Once an easy handle has been added to a multi handle then it can no longer"] # [doc = " be used via `perform`. This handle is also used to remove the easy handle"] # [doc = " from the multi handle when desired."] pub struct EasyHandle { guard : DetachGuard , easy : Easy , _marker : marker :: PhantomData < & 'static Multi > , }
    };
}

EasyHandle!()