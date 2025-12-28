macro_rules! macro_480 {
    () => {
        pin_project ! { # [doc = " Stream for the [`chunks`](super::StreamExt::chunks) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Chunks < St : Stream > { # [pin] stream : Fuse < St >, items : Vec < St :: Item >, cap : usize , } }
    };
}

macro_480!()