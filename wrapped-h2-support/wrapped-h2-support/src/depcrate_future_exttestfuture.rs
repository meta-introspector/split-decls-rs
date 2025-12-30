// Generated macro for TestFuture (trait)
macro_rules! Depcrate_future_extTestFuture {
() => {
// Module: crate::future_ext
// Provides: {"TestFuture"}
// Dependencies: {}
# [doc = " Future extension helpers that are useful for tests"] pub trait TestFuture : Future { # [doc = " Drive `other` by polling `self`."] # [doc = ""] # [doc = " `self` must not resolve before `other` does."] fn drive < T > (& mut self , other : T) -> Drive < '_ , Self , T > where T : Future , Self : Future + Sized , { Drive { driver : self , future : other . wakened () , } } fn wakened (self) -> Wakened < Self > where Self : Sized , { Wakened { future : Box :: pin (self) , woken : Arc :: new (AtomicBool :: new (true)) , } } }
};
}
