macro_rules! macro_286 {
    () => {
        pin_project ! { # [doc = " Future for the [`collect`](super::StreamExt::collect) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Collect < St , C > { # [pin] stream : St , collection : C , } }
    };
}

macro_286!()