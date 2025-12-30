// Generated macro for ItertoolsNum (trait)
macro_rules! Depcrate_extItertoolsNum {
() => {
// Module: crate::ext
// Provides: {"ItertoolsNum"}
// Dependencies: {}
# [doc = " Extension trait for iterators: extra adaptors and methods"] # [doc = " for numerical iterators"] pub trait ItertoolsNum : Iterator { # [doc = " Return an iterator that produces the sequence of cumulative sums"] # [doc = " of the base iterator. The type of the sum is `S`."] fn cumsum < S > (self) -> Cumsum < Self , S > where Self : Sized , S : Add < Self :: Item , Output = S > , S : Zero , { cumsum (self) } }
};
}
