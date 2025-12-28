macro_rules! Drain {
    () => {
        # [doc = " Stream for the [`StreamExt::drain()`] method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Drain < 'a , S : ? Sized > { stream : & 'a mut S , }
    };
}

Drain!();