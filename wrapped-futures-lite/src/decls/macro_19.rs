macro_rules! macro_19 {
    () => {
        pin_project ! { # [doc = " Future for the [`try_zip()`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryZip < F1 , T1 , F2 , T2 > { # [pin] future1 : Option < F1 >, output1 : Option < T1 >, # [pin] future2 : Option < F2 >, output2 : Option < T2 >, } }
    };
}

macro_19!();