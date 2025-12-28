macro_rules! macro_22 {
    () => {
        pin_project ! { # [doc = " Future for the [`or()`] function and the [`FutureExt::or()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Or < F1 , F2 > { # [pin] future1 : F1 , # [pin] future2 : F2 , } }
    };
}

macro_22!()