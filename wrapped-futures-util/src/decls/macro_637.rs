macro_rules! macro_637 {
    () => {
        pin_project ! { # [doc = " Stream for the [`try_flatten`](super::TryStreamExt::try_flatten) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct TryFlatten < St > where St : TryStream , { # [pin] stream : St , # [pin] next : Option < St :: Ok >, } }
    };
}

macro_637!();