// Generated macro for smallvec (macro)
macro_rules! Depcratesmallvec {
() => {
// Module: crate
// Provides: {"smallvec"}
// Dependencies: {}
# [macro_export] macro_rules ! smallvec { (@ one $ x : expr) => (1usize) ; () => ($ crate :: SmallVec :: new ()) ; ($ elem : expr ; $ n : expr) => ({ $ crate :: from_elem ($ elem , $ n) }) ; ($ ($ x : expr) ,+$ (,) ?) => ({ const COUNT : usize = 0usize $ (+ $ crate :: smallvec ! (@ one $ x)) +; let mut vec = $ crate :: SmallVec :: new () ; if COUNT <= vec . capacity () { $ (vec . push ($ x) ;) * vec } else { $ crate :: SmallVec :: from_vec ($ crate :: alloc :: vec ! [$ ($ x ,) +]) } }) ; }
};
}
