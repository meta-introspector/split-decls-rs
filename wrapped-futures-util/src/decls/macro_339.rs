macro_rules! macro_339 {
    () => {
        pin_project ! { # [doc = " Future for the [`fold`](super::StreamExt::fold) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Fold < St , Fut , T , F > { # [pin] stream : St , f : F , accum : Option < T >, # [pin] future : Option < Fut >, } }
    };
}

macro_339!();