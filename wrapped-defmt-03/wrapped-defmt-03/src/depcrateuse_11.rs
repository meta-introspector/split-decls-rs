// Generated macro for use_11 (pub_use)
macro_rules! Depcrateuse_11 {
() => {
// Module: crate
// Provides: {"use_11"}
// Dependencies: {}
# [doc = " Unwraps an `Option` or `Result`, panicking if it is `None` or `Err`."] # [doc = ""] # [doc = " This macro is roughly equivalent to `{Option,Result}::{expect,unwrap}` but invocation looks"] # [doc = " a bit different because this is a macro and not a method. The other difference is that"] # [doc = " `unwrap!`-ing a `Result<T, E>` value requires that the error type `E` implements the `Format`"] # [doc = " trait"] # [doc = ""] # [doc = " The following snippet shows the differences between core's unwrap method and defmt's unwrap"] # [doc = " macro:"] # [doc = ""] # [doc = " ```"] # [doc = " use defmt::unwrap;"] # [doc = ""] # [doc = " # let option = Some(());"] # [doc = " let x = option.unwrap();"] # [doc = " let x = unwrap!(option);"] # [doc = ""] # [doc = " # let result = Ok::<(), ()>(());"] # [doc = " let x = result.unwrap();"] # [doc = " let x = unwrap!(result);"] # [doc = ""] # [doc = " let x = result.expect(\"text\");"] # [doc = " let x = unwrap!(result, \"text\");"] # [doc = ""] # [doc = " # let arg = ();"] # [doc = " let x = result.expect(&format!(\"text {:?}\", arg));"] # [doc = " let x = unwrap!(result, \"text {:?}\", arg); // arg must be implement `Format`"] # [doc = " ```"] # [doc = ""] # [doc = " If used, the format string must follow the defmt syntax (documented in [the manual])"] # [doc = ""] # [doc = " [the manual]: https://defmt.ferrous-systems.com/macros.html"] pub use defmt10 :: unwrap ;
};
}
