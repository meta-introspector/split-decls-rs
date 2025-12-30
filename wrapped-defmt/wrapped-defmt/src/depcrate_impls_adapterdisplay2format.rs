// Generated macro for Display2Format (struct)
macro_rules! Depcrate_impls_adapterDisplay2Format {
() => {
// Module: crate::impls::adapter
// Provides: {"Display2Format"}
// Dependencies: {}
# [doc = " An \"adapter\" type to feed `Display` values into defmt macros, which expect `defmt::Format` values."] # [doc = ""] # [doc = " This adapter disables compression and uses the `core::fmt` code on-device! You should prefer"] # [doc = " `defmt::Format` over `Display` whenever possible."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # struct ExpensiveThing();"] # [doc = " #"] # [doc = " # impl core::fmt::Display for ExpensiveThing {"] # [doc = " #     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {"] # [doc = " #         write!(f, \"{}\", \"expensive\")"] # [doc = " #     }"] # [doc = " #  }"] # [doc = " # let expensive_thing = ExpensiveThing();"] # [doc = " #"] # [doc = " defmt::info!(\"{}\", defmt::Display2Format(&expensive_thing));"] # [doc = " //                                        ˆˆˆˆˆˆˆˆˆˆˆˆˆˆˆ"] # [doc = " //                                        must implement `fmt::Display`"] # [doc = " ```"] # [doc = ""] # [doc = " Note that any provided defmt display hints will be ignored"] # [doc = " because this always uses `{}` to format the contained value."] pub struct Display2Format < 'a , T : fmt :: Display + ? Sized > (pub & 'a T) ;
};
}
