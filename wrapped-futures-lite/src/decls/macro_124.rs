macro_rules! macro_124 {
    () => {
        pin_project ! { # [doc = " Stream for the [`or()`] function and the [`StreamExt::or()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Or < S1 , S2 > { # [pin] stream1 : S1 , # [pin] stream2 : S2 , } }
    };
}

macro_124!();