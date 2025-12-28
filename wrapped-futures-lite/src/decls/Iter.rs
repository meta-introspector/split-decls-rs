macro_rules! Iter {
    () => {
        # [doc = " Stream for the [`iter()`] function."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Iter < I > { iter : I , }
    };
}

Iter!();