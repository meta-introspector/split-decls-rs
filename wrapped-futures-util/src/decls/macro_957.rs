macro_rules! macro_957 {
    () => {
        pin_project ! { # [doc = " Future for the [`send_all`](super::SinkExt::send_all) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct SendAll <'a , Si , St > where Si : ? Sized , St : TryStream , { sink : &'a mut Si , # [pin] stream : Fuse < St >, buffered : Option < St :: Ok >, } }
    };
}

macro_957!()