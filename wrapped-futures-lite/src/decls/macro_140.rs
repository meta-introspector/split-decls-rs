macro_rules! macro_140 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::skip_while()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct SkipWhile < S , P > { # [pin] stream : S , predicate : Option < P >, } }
    };
}

macro_140!()