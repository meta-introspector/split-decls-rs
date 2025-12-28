macro_rules! macro_1100 {
    () => {
        pin_project ! { # [doc = " Future for the [`copy_buf()`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CopyBuf <'a , R , W : ? Sized > { # [pin] reader : R , writer : &'a mut W , amt : u64 , } }
    };
}

macro_1100!()