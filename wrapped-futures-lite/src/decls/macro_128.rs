macro_rules! macro_128 {
    () => {
        # [cfg (feature = "race")] pin_project ! { # [doc = " Stream for the [`race()`] function and the [`StreamExt::race()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Race < S1 , S2 > { # [pin] stream1 : S1 , # [pin] stream2 : S2 , rng : Rng , } }
    };
}

macro_128!()