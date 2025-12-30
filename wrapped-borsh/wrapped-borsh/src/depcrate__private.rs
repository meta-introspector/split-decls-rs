// Generated macro for __private (module)
macro_rules! Depcrate__private {
() => {
// Module: crate
// Provides: {"__private"}
// Dependencies: {}
# [doc (hidden)] pub mod __private { # [doc = " A facade around all the types we need from the `std`, and `alloc`"] # [doc = " crates. This avoids elaborate import wrangling having to happen in every"] # [doc = " module."] # [cfg (feature = "std")] pub mod maybestd { pub use std :: { borrow , boxed , collections , format , string , vec } ; # [cfg (feature = "rc")] pub use std :: { rc , sync } ; } # [cfg (not (feature = "std"))] pub mod maybestd { pub use alloc :: { borrow , boxed , format , string , vec } ; # [cfg (feature = "rc")] pub use alloc :: { rc , sync } ; pub mod collections { pub use alloc :: collections :: { btree_map , BTreeMap , BTreeSet , LinkedList , VecDeque } ; # [cfg (feature = "hashbrown")] pub use hashbrown :: * ; } } }
};
}
