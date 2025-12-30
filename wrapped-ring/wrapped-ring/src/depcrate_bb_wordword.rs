// Generated macro for Word (type)
macro_rules! Depcrate_bb_wordWord {
() => {
// Module: crate::bb::word
// Provides: {"Word"}
// Dependencies: {}
# [doc = " A native word that may hold a secret."] # [doc = ""] # [doc = " XXX: Currently this is a type alias of `LeakyWord` so it doesn't enforce,"] # [doc = " except by convention, the prevention of leaks. This is a temporary state to"] # [doc = " support the refactorings that will"] # [doc = ""] # [doc = " XXX: This isn't the native word size on targets where a pointer isn't the"] # [doc = " same size as a native word. TODO: Fix this."] # [doc = ""] # [doc = " XXX: Over time, we'll evolve Word into a newtype with an API that minimizes"] # [doc = " leaks and makes all leaks explicit, like so:"] pub (crate) type Word = LeakyWord ;
};
}
