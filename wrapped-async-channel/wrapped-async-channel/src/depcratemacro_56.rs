// Generated macro for macro_56 (macro)
macro_rules! Depcratemacro_56 {
() => {
// Module: crate
// Provides: {"macro_56"}
// Dependencies: {}
easy_wrapper ! { # [doc = " A future returned by [`Sender::send()`]."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Send <'a , T > (SendInner <'a , T > => Result < () , SendError < T >>) ; # [cfg (all (feature = "std" , not (target_family = "wasm")))] pub (crate) wait () ; }
};
}
