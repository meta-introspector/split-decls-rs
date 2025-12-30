// Generated macro for Builder (struct)
macro_rules! Depcrate_shortcutsBuilder {
() => {
// Module: crate::shortcuts
// Provides: {"Builder"}
// Dependencies: {}
struct Builder < 'a , 'b > { lexed : & 'a LexedStr < 'a > , pos : usize , state : State , sink : & 'b mut dyn FnMut (StrStep < '_ >) , }
};
}
