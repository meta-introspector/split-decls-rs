macro_rules! macro_596 {
    () => {
        pin_project ! { # [doc = " Stream for the [`into_stream`](super::TryStreamExt::into_stream) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct IntoStream < St > { # [pin] stream : St , } }
    };
}

macro_596!();