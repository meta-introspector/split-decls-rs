// Generated macro for finished (function)
macro_rules! Depcrate_finishedfinished {
() => {
// Module: crate::finished
// Provides: {"finished"}
// Dependencies: {}
# [doc = " Creates a \"leaf future\" from an immediate value of a finished and"] # [doc = " successful computation."] # [doc = ""] # [doc = " The returned future is similar to `done` where it will immediately run a"] # [doc = " scheduled callback with the provided value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::*;"] # [doc = ""] # [doc = " let future_of_1 = finished::<u32, u32>(1);"] # [doc = " ```"] pub fn finished < T , E > (t : T) -> Finished < T , E > where T : Send + 'static , E : Send + 'static , { Finished { t : Some (t) , _e : marker :: PhantomData } }
};
}
