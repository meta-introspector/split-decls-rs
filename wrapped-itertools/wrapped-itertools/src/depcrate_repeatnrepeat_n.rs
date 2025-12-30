// Generated macro for repeat_n (function)
macro_rules! Depcrate_repeatnrepeat_n {
() => {
// Module: crate::repeatn
// Provides: {"repeat_n"}
// Dependencies: {}
# [doc = " Create an iterator that produces `n` repetitions of `element`."] pub fn repeat_n < A > (element : A , n : usize) -> RepeatN < A > where A : Clone , { if n == 0 { RepeatN { elt : None , n } } else { RepeatN { elt : Some (element) , n , } } }
};
}
