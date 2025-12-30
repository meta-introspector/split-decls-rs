// Generated macro for __private (module)
macro_rules! Depcrate__private {
() => {
// Module: crate
// Provides: {"__private"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "std")] pub mod __private { pub use futures_core :: { future , stream , task } ; pub use futures_executor :: block_on ; pub use std :: { option :: Option :: { None , Some } , pin :: Pin , result :: Result :: { Err , Ok } , } ; pub mod assert { pub use crate :: assert :: * ; } }
};
}
