// Generated macro for IntoIter (struct)
macro_rules! DepcrateIntoIter {
() => {
// Module: crate
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " By-value `ArrayDeque` iterator"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct IntoIter < T , const CAP : usize , B : Behavior > { inner : ArrayDeque < T , CAP , B > , }
};
}
