// Generated macro for macro_62 (macro)
macro_rules! Depcratemacro_62 {
() => {
// Module: crate
// Provides: {"macro_62"}
// Dependencies: {}
easy_wrapper ! { # [doc = " A future returned by [`Sender::closed()`]."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Closed <'a , T > (ClosedInner <'a , T > => ()) ; # [cfg (all (feature = "std" , not (target_family = "wasm")))] pub (crate) wait () ; }
};
}
