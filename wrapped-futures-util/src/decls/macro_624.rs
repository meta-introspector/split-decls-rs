macro_rules! macro_624 {
    () => {
        pin_project ! { # [doc = " Future for the [`try_forward`](super::TryStreamExt::try_forward) method."] # [project = TryForwardProj] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryForward < St , Si , Item > { # [pin] sink : Option < Si >, # [pin] stream : Fuse < IntoStream < St >>, buffered_item : Option < Item >, } }
    };
}

macro_624!()