macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! Ok {
    () => {
        deps!();
        # [doc = " Equivalent to `Ok::<_, anyhow::Error>(value)`."] # [doc = ""] # [doc = " This simplifies creation of an `anyhow::Result` in places where type"] # [doc = " inference cannot deduce the `E` type of the result &mdash; without needing"] # [doc = " to write `Ok::<_, anyhow::Error>(value)`."] # [doc = ""] # [doc = " One might think that `anyhow::Result::Ok(value)` would work in such cases"] # [doc = " but it does not."] # [doc = ""] # [doc = " ```console"] # [doc = " error[E0282]: type annotations needed for `std::result::Result<i32, E>`"] # [doc = "   --> src/main.rs:11:13"] # [doc = "    |"] # [doc = " 11 |     let _ = anyhow::Result::Ok(1);"] # [doc = "    |         -   ^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `E` declared on the enum `Result`"] # [doc = "    |         |"] # [doc = "    |         consider giving this pattern the explicit type `std::result::Result<i32, E>`, where the type parameter `E` is specified"] # [doc = " ```"] # [allow (non_snake_case)] pub fn Ok < T > (value : T) -> Result < T > { Result :: Ok (value) }
    };
}

Ok!()