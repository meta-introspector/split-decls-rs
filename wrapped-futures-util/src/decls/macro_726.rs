macro_rules! macro_726 {
    () => {
        pin_project ! { # [doc = " Future for the [`try_all`](super::TryStreamExt::try_all) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryAll < St , Fut , F > { # [pin] stream : St , f : F , done : bool , # [pin] future : Option < Fut >, } }
    };
}

macro_726!();