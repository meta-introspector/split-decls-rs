// Generated macro for macro_18 (macro)
macro_rules! Depcrate_future_eithermacro_18 {
() => {
// Module: crate::future::either
// Provides: {"macro_18"}
// Dependencies: {}
pin_project ! { # [doc = " Combines two different futures that have the same output type."] # [doc = ""] # [doc = " Construct variants with [`Either::left`] and [`Either::right`]."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use actix_utils::future::{ready, Ready, Either};"] # [doc = ""] # [doc = " # async fn run() {"] # [doc = " let res = Either::<_, Ready<usize>>::left(ready(42));"] # [doc = " assert_eq!(res.await, 42);"] # [doc = ""] # [doc = " let res = Either::<Ready<usize>, _>::right(ready(43));"] # [doc = " assert_eq!(res.await, 43);"] # [doc = " # }"] # [doc = " ```"] # [project = EitherProj] # [derive (Debug , Clone)] pub enum Either < L , R > { # [doc = " A value of type `L`."] # [allow (missing_docs)] Left { # [pin] value : L } , # [doc = " A value of type `R`."] # [allow (missing_docs)] Right { # [pin] value : R } , } }
};
}
