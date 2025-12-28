macro_rules! macro_291 {
    () => {
        pin_project ! { # [doc = " Future for the [`unzip`](super::StreamExt::unzip) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Unzip < St , FromA , FromB > { # [pin] stream : St , left : FromA , right : FromB , } }
    };
}

macro_291!()