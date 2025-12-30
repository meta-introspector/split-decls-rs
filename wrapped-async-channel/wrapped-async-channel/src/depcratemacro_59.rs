// Generated macro for macro_59 (macro)
macro_rules! Depcratemacro_59 {
() => {
// Module: crate
// Provides: {"macro_59"}
// Dependencies: {}
easy_wrapper ! { # [doc = " A future returned by [`Receiver::recv()`]."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Recv <'a , T > (RecvInner <'a , T > => Result < T , RecvError >) ; # [cfg (all (feature = "std" , not (target_family = "wasm")))] pub (crate) wait () ; }
};
}
