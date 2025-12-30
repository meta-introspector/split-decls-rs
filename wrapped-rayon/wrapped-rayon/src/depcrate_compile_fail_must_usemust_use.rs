// Generated macro for must_use (macro)
macro_rules! Depcrate_compile_fail_must_usemust_use {
() => {
// Module: crate::compile_fail::must_use
// Provides: {"must_use"}
// Dependencies: {}
macro_rules ! must_use { ($ ($ name : ident # [$ expr : meta]) *) => { $ (# [doc = " First sanity check that the expression is OK."] # [doc = ""] # [doc = " ```"] # [doc = " #![deny(unused_must_use)]"] # [doc = ""] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " let v: Vec<_> = (0..100).map(Some).collect();"] # [doc = " let _ ="] # [$ expr] # [doc = " ```"] # [doc = ""] # [doc = " Now trigger the `must_use`."] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " #![deny(unused_must_use)]"] # [doc = ""] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " let v: Vec<_> = (0..100).map(Some).collect();"] # [$ expr] # [doc = " ```"] mod $ name { }) * } }
};
}
