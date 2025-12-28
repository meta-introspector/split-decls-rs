macro_rules! macro_816 {
    () => {
        pin_project ! { # [must_use = "futures do nothing unless you `.await` or poll them"] # [derive (Debug)] struct OrderWrapper < T > { # [pin] data : T , index : i64 , } }
    };
}

macro_816!();