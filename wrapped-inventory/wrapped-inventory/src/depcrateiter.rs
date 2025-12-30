// Generated macro for iter (type)
macro_rules! Depcrateiter {
() => {
// Module: crate
// Provides: {"iter"}
// Dependencies: {}
# [doc = " An iterator over plugins registered of a given type."] # [doc = ""] # [doc = " The value `inventory::iter::<T>` is an iterator with element type `&'static"] # [doc = " T`."] # [doc = ""] # [doc = " There is no guarantee about the order that plugins of the same type are"] # [doc = " visited by the iterator. They may be visited in any order."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # struct Flag {"] # [doc = " #     short: char,"] # [doc = " #     name: &'static str,"] # [doc = " # }"] # [doc = " #"] # [doc = " # inventory::collect!(Flag);"] # [doc = " #"] # [doc = " # const IGNORE: &str = stringify! {"] # [doc = " use my_flags::Flag;"] # [doc = " # };"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     for flag in inventory::iter::<Flag> {"] # [doc = "         println!(\"-{}, --{}\", flag.short, flag.name);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Refer to the [crate level documentation](index.html) for a complete example"] # [doc = " of instantiating a plugin registry and submitting plugins."] # [allow (non_camel_case_types)] pub type iter < T > = private :: iter < T > ;
};
}
