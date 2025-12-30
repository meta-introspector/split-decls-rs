// Generated macro for macro_378 (macro)
macro_rules! Depcrate_future_joinmacro_378 {
() => {
// Module: crate::future::join
// Provides: {"macro_378"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`join`](super::FutureExt::join) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Join < Fut1 , Fut2 > where Fut1 : Future , Fut2 : Future { # [pin] fut1 : MaybeDone < Fut1 >, # [pin] fut2 : MaybeDone < Fut2 >, } }
};
}
