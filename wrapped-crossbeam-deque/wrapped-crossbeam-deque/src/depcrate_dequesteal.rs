// Generated macro for Steal (enum)
macro_rules! Depcrate_dequeSteal {
() => {
// Module: crate::deque
// Provides: {"Steal"}
// Dependencies: {}
# [doc = " Possible outcomes of a steal operation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " There are lots of ways to chain results of steal operations together:"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_deque::Steal::{self, Empty, Retry, Success};"] # [doc = ""] # [doc = " let collect = |v: Vec<Steal<i32>>| v.into_iter().collect::<Steal<i32>>();"] # [doc = ""] # [doc = " assert_eq!(collect(vec![Empty, Empty, Empty]), Empty);"] # [doc = " assert_eq!(collect(vec![Empty, Retry, Empty]), Retry);"] # [doc = " assert_eq!(collect(vec![Retry, Success(1), Empty]), Success(1));"] # [doc = ""] # [doc = " assert_eq!(collect(vec![Empty, Empty]).or_else(|| Retry), Retry);"] # [doc = " assert_eq!(collect(vec![Retry, Empty]).or_else(|| Success(1)), Success(1));"] # [doc = " ```"] # [must_use] # [derive (PartialEq , Eq , Copy , Clone)] pub enum Steal < T > { # [doc = " The queue was empty at the time of stealing."] Empty , # [doc = " At least one task was successfully stolen."] Success (T) , # [doc = " The steal operation needs to be retried."] Retry , }
};
}
