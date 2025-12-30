// Generated macro for Ok (function)
macro_rules! DepcrateOk {
() => {
// Module: crate
// Provides: {"Ok"}
// Dependencies: {}
# [doc = " Equivalent to `Ok::<_, eyre::Error>(value)`."] # [doc = ""] # [doc = " This simplifies creation of an eyre::Result in places where type inference"] # [doc = " cannot deduce the `E` type of the result &mdash; without needing to write"] # [doc = " `Ok::<_, eyre::Error>(value)`."] # [doc = ""] # [doc = " One might think that `eyre::Result::Ok(value)` would work in such cases"] # [doc = " but it does not."] # [doc = ""] # [doc = " ```console"] # [doc = " error[E0282]: type annotations needed for `std::result::Result<i32, E>`"] # [doc = "   --> src/main.rs:11:13"] # [doc = "    |"] # [doc = " 11 |     let _ = eyre::Result::Ok(1);"] # [doc = "    |         -   ^^^^^^^^^^^^^^^^ cannot infer type for type parameter `E` declared on the enum `Result`"] # [doc = "    |         |"] # [doc = "    |         consider giving this pattern the explicit type `std::result::Result<i32, E>`, where the type parameter `E` is specified"] # [doc = " ```"] # [allow (non_snake_case)] pub fn Ok < T > (t : T) -> Result < T > { Result :: Ok (t) }
};
}
