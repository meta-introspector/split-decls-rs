macro_rules! macro_732 {
    () => {
        pin_project ! { # [doc = " Future for the [`try_any`](super::TryStreamExt::try_any) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryAny < St , Fut , F > { # [pin] stream : St , f : F , done : bool , # [pin] future : Option < Fut >, } }
    };
}

macro_732!();