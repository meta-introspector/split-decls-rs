// Generated macro for repeat (function)
macro_rules! Depcraterepeat {
() => {
// Module: crate
// Provides: {"repeat"}
// Dependencies: {}
# [doc = " Creates an iterator that endlessly repeats a single element."] pub fn repeat < T : Clone , E > (value : T) -> Repeat < T , E > { Repeat (value , PhantomData) }
};
}
