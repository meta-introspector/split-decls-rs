macro_rules! macro_1216 {
    () => {
        pin_project ! { # [doc = " Reader for the [`take`](super::AsyncReadExt::take) method."] # [derive (Debug)] # [must_use = "readers do nothing unless you `.await` or poll them"] pub struct Take < R > { # [pin] inner : R , limit : u64 , } }
    };
}

macro_1216!()