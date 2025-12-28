macro_rules! Pending {
    () => {
        # [doc = " Stream for the [`pending()`] function."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Pending < T > { _data : marker :: PhantomData < T > , }
    };
}

Pending!()