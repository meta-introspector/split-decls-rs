macro_rules! deps {
    () => {
        String!();
    };
}

macro_rules! format {
    () => {
        deps!();
        # [doc = " Macro that creates a fixed capacity [`String`]. Equivalent to [`format!`](https://doc.rust-lang.org/std/macro.format.html)."] # [doc = ""] # [doc = " The macro's arguments work in the same way as the regular macro."] # [doc = ""] # [doc = " It is possible to explicitly specify the capacity of the returned string as the first argument."] # [doc = " In this case it is necessary to disambiguate by separating the capacity with a semicolon."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " There are two possible error cases. Both return the unit type [`core::fmt::Error`]."] # [doc = ""] # [doc = " - In case the formatting exceeds the string's capacity. This error does not exist in the"] # [doc = "   standard library as the string would just grow."] # [doc = " - If a formatting trait implementation returns an error. The standard library panics in this"] # [doc = "   case."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # fn main() -> Result<(), core::fmt::Error> {"] # [doc = " use heapless::{format, String};"] # [doc = ""] # [doc = " // Notice semicolon instead of comma!"] # [doc = " format!(4; \"test\")?;"] # [doc = " format!(15; \"hello {}\", \"world!\")?;"] # [doc = " format!(20; \"x = {}, y = {y}\", 10, y = 30)?;"] # [doc = " let (x, y) = (1, 2);"] # [doc = " format!(12; \"{x} + {y} = 3\")?;"] # [doc = ""] # [doc = " let implicit: String<10> = format!(\"speed = {}\", 7)?;"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [macro_export] macro_rules ! format { ($ max : expr ; $ lenT : path ; $ ($ arg : tt) *) => { { let res = $ crate :: _export :: format ::<$ max , $ lenT > (core :: format_args ! ($ ($ arg) *)) ; res } } ; ($ max : expr ; $ ($ arg : tt) *) => { { let res = $ crate :: _export :: format ::<$ max , usize > (core :: format_args ! ($ ($ arg) *)) ; res } } ; ($ ($ arg : tt) *) => { { let res = $ crate :: _export :: format (core :: format_args ! ($ ($ arg) *)) ; res } } ; }
    };
}

format!()