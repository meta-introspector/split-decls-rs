// Generated macro for macro_7052 (macro)
macro_rules! Depcrate_methodsmacro_7052 {
() => {
// Module: crate::methods
// Provides: {"macro_7052"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects `().hash(_)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Hashing a unit value doesn't do anything as the implementation of `Hash` for `()` is a no-op."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::hash::Hash;"] # [doc = " # use std::collections::hash_map::DefaultHasher;"] # [doc = " # enum Foo { Empty, WithValue(u8) }"] # [doc = " # use Foo::*;"] # [doc = " # let mut state = DefaultHasher::new();"] # [doc = " # let my_enum = Foo::Empty;"] # [doc = " match my_enum {"] # [doc = " \tEmpty => ().hash(&mut state),"] # [doc = " \tWithValue(x) => x.hash(&mut state),"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::hash::Hash;"] # [doc = " # use std::collections::hash_map::DefaultHasher;"] # [doc = " # enum Foo { Empty, WithValue(u8) }"] # [doc = " # use Foo::*;"] # [doc = " # let mut state = DefaultHasher::new();"] # [doc = " # let my_enum = Foo::Empty;"] # [doc = " match my_enum {"] # [doc = " \tEmpty => 0_u8.hash(&mut state),"] # [doc = " \tWithValue(x) => x.hash(&mut state),"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.58.0"] pub UNIT_HASH , correctness , "hashing a unit value, which does nothing" }
};
}
