macro_rules! macro_136 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::map_while()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct MapWhile < S , P > { # [pin] stream : S , predicate : P , } }
    };
}

macro_136!()