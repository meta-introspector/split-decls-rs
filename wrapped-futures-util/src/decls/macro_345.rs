macro_rules! macro_345 {
    () => {
        pin_project ! { # [doc = " Future for the [`any`](super::StreamExt::any) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Any < St , Fut , F > { # [pin] stream : St , f : F , done : bool , # [pin] future : Option < Fut >, } }
    };
}

macro_345!();