macro_rules! macro_296 {
    () => {
        pin_project ! { # [doc = " Future for the [`concat`](super::StreamExt::concat) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Concat < St : Stream > { # [pin] stream : St , accum : Option < St :: Item >, } }
    };
}

macro_296!()