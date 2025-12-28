macro_rules! macro_469 {
    () => {
        pin_project ! { # [doc = " Future for the [`try_fold`](super::TryStreamExt::try_fold) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryFold < St , Fut , T , F > { # [pin] stream : St , f : F , accum : Option < T >, # [pin] future : Option < Fut >, } }
    };
}

macro_469!();