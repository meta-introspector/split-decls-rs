// Generated macro for once (function)
macro_rules! Depcrateonce {
() => {
// Module: crate
// Provides: {"once"}
// Dependencies: {}
# [doc = " Creates an iterator that yields an element exactly once."] pub fn once < T , E > (value : T) -> Once < T , E > { Once (Some (value) , PhantomData) }
};
}
