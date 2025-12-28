macro_rules! PollFn {
    () => {
        # [doc = " Stream for the [`poll_fn()`] function."] # [derive (Clone)] # [must_use = "streams do nothing unless polled"] pub struct PollFn < F > { f : F , }
    };
}

PollFn!();