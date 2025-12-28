macro_rules! macro_369 {
    () => {
        pin_project ! { # [doc = " Stream for the [`fuse`](super::StreamExt::fuse) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Fuse < St > { # [pin] stream : St , done : bool , } }
    };
}

macro_369!()