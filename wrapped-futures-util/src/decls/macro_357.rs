macro_rules! macro_357 {
    () => {
        pin_project ! { # [doc = " Future for the [`forward`](super::StreamExt::forward) method."] # [project = ForwardProj] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Forward < St , Si , Item > { # [pin] sink : Option < Si >, # [pin] stream : Fuse < St >, buffered_item : Option < Item >, } }
    };
}

macro_357!();