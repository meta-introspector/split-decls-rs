macro_rules! macro_1096 {
    () => {
        pin_project ! { # [doc = " Future for the [`copy()`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Copy <'a , R , W : ? Sized > { # [pin] inner : CopyBuf <'a , BufReader < R >, W >, } }
    };
}

macro_1096!();