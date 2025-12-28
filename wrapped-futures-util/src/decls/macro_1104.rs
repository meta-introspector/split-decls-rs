macro_rules! deps {
    () => {
        AbortInner!();
    };
}

macro_rules! macro_1104 {
    () => {
        deps!();
        pin_project ! { # [doc = " Future for the [`copy_buf_abortable()`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CopyBufAbortable <'a , R , W : ? Sized > { # [pin] reader : R , writer : &'a mut W , amt : u64 , inner : Arc < AbortInner > } }
    };
}

macro_1104!();