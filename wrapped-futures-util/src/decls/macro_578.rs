macro_rules! macro_578 {
    () => {
        pin_project ! { # [doc = " Stream for the [`catch_unwind`](super::StreamExt::catch_unwind) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct CatchUnwind < St > { # [pin] stream : St , caught_unwind : bool , } }
    };
}

macro_578!()