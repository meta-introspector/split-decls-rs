// Generated macro for ValueRef (trait)
macro_rules! DepcrateValueRef {
() => {
// Module: crate
// Provides: {"ValueRef"}
// Dependencies: {}
# [doc = "\nA producer of structured data that stores a reference internally.\n\nThis trait is a variant of [`Value`] for wrapper types that keep a reference to a value internally.\nIn `Value`, the `'sval` lifetime comes from the borrow of `&'sval self`. In `ValueRef`, it comes\nfrom the `'sval` lifetime in the trait itself.\n"] pub trait ValueRef < 'sval > : Value { # [doc = "\n    Stream this value through a [`Stream`].\n    "] fn stream_ref < S : Stream < 'sval > + ? Sized > (& self , stream : & mut S) -> Result ; }
};
}
