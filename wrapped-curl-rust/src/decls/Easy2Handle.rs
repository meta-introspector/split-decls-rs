macro_rules! deps {
    () => {
        Easy2!();
        Multi!();
        DetachGuard!();
    };
}

macro_rules! Easy2Handle {
    () => {
        deps!();
        # [doc = " Wrapper around an easy handle while it's owned by a multi handle."] # [doc = ""] # [doc = " Once an easy handle has been added to a multi handle then it can no longer"] # [doc = " be used via `perform`. This handle is also used to remove the easy handle"] # [doc = " from the multi handle when desired."] pub struct Easy2Handle < H > { guard : DetachGuard , easy : Easy2 < H > , _marker : marker :: PhantomData < & 'static Multi > , }
    };
}

Easy2Handle!()