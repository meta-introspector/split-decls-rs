macro_rules! deps {
    () => {
        MaybeDone!();
    };
}

macro_rules! macro_209 {
    () => {
        deps!();
        pin_project ! { # [doc = " Future for the [`join`](super::FutureExt::join) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Join < Fut1 , Fut2 > where Fut1 : Future , Fut2 : Future { # [pin] fut1 : MaybeDone < Fut1 >, # [pin] fut2 : MaybeDone < Fut2 >, } }
    };
}

macro_209!();