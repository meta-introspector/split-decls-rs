macro_rules! macro_475 {
    () => {
        pin_project ! { # [doc = " Stream for the [`zip`](super::StreamExt::zip) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Zip < St1 : Stream , St2 : Stream > { # [pin] stream1 : Fuse < St1 >, # [pin] stream2 : Fuse < St2 >, queued1 : Option < St1 :: Item >, queued2 : Option < St2 :: Item >, } }
    };
}

macro_475!()