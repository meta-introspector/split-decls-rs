// Generated macro for RepeatN (struct)
macro_rules! Depcrate_repeatnRepeatN {
() => {
// Module: crate::repeatn
// Provides: {"RepeatN"}
// Dependencies: {}
# [doc = " An iterator that produces *n* repetitions of an element."] # [doc = ""] # [doc = " See [`repeat_n()`](crate::repeat_n) for more information."] # [must_use = "iterators are lazy and do nothing unless consumed"] # [derive (Clone , Debug)] pub struct RepeatN < A > { pub (crate) elt : Option < A > , n : usize , }
};
}
