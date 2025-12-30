// Generated macro for Drain (struct)
macro_rules! DepcrateDrain {
() => {
// Module: crate
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " Draining `ArrayDeque` iterator"] pub struct Drain < 'a , T , const CAP : usize , B > where B : Behavior , { after_tail : usize , after_len : usize , iter : Iter < 'a , T > , deque : * mut ArrayDeque < T , CAP , B > , }
};
}
