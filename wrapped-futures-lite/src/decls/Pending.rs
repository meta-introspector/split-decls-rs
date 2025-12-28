macro_rules! Pending {
    () => {
        # [doc = " Stream for the [`pending()`] function."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Pending < T > { _marker : PhantomData < T > , }
    };
}

Pending!()