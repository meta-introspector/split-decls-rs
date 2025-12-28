macro_rules! macro_301 {
    () => {
        pin_project ! { # [doc = " Future for the [`count`](super::StreamExt::count) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Count < St > { # [pin] stream : St , count : usize } }
    };
}

macro_301!();