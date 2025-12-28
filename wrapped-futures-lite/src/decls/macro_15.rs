macro_rules! macro_15 {
    () => {
        pin_project ! { # [doc = " Future for the [`zip()`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Zip < F1 , F2 > where F1 : Future , F2 : Future , { # [pin] future1 : Option < F1 >, output1 : Option < F1 :: Output >, # [pin] future2 : Option < F2 >, output2 : Option < F2 :: Output >, } }
    };
}

macro_15!()