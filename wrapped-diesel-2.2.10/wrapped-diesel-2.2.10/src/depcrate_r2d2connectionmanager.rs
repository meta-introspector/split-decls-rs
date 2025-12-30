// Generated macro for ConnectionManager (struct)
macro_rules! Depcrate_r2d2ConnectionManager {
() => {
// Module: crate::r2d2
// Provides: {"ConnectionManager"}
// Dependencies: {}
# [doc = " An r2d2 connection manager for use with Diesel."] # [doc = ""] # [doc = " See the [r2d2 documentation](https://docs.rs/r2d2/latest/r2d2/) for usage examples."] # [derive (Clone)] pub struct ConnectionManager < T > { database_url : String , _marker : PhantomData < T > , }
};
}
