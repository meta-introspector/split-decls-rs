// Generated macro for repeat_err (function)
macro_rules! Depcraterepeat_err {
() => {
// Module: crate
// Provides: {"repeat_err"}
// Dependencies: {}
# [doc = " Creates an iterator that endlessly repeats a single error."] pub fn repeat_err < T , E : Clone > (value : E) -> RepeatErr < T , E > { RepeatErr (PhantomData , value) }
};
}
