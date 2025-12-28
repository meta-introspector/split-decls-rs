macro_rules! Unfold {
    () => {
        # [doc = " See [`unfold`](crate::unfold) for more information."] # [derive (Clone)] # [must_use = "iterators are lazy and do nothing unless consumed"] # [deprecated (note = "Use [std::iter::FromFn](https://doc.rust-lang.org/std/iter/struct.FromFn.html) instead" , since = "0.13.0")] pub struct Unfold < St , F > { f : F , # [doc = " Internal state that will be passed to the closure on the next iteration"] pub state : St , }
    };
}

Unfold!()