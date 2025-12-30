// Generated macro for macro_313 (macro)
macro_rules! Depcrate_future_optionmacro_313 {
() => {
// Module: crate::future::option
// Provides: {"macro_313"}
// Dependencies: {}
pin_project ! { # [doc = " A future representing a value which may or may not be present."] # [doc = ""] # [doc = " Created by the [`From`] implementation for [`Option`](std::option::Option)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::future::OptionFuture;"] # [doc = ""] # [doc = " let mut a: OptionFuture<_> = Some(async { 123 }).into();"] # [doc = " assert_eq!(a.await, Some(123));"] # [doc = ""] # [doc = " a = None.into();"] # [doc = " assert_eq!(a.await, None);"] # [doc = " # });"] # [doc = " ```"] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct OptionFuture < F > { # [pin] inner : Option < F >, } }
};
}
