macro_rules! deps {
    () => {
        Var!();
    };
}

macro_rules! Env {
    () => {
        deps!();
        # [doc = " Set of environment variables to configure from."] # [doc = ""] # [doc = " # Default environment variables"] # [doc = ""] # [doc = " By default, the `Env` will read the following environment variables:"] # [doc = ""] # [doc = " - `RUST_LOG`: the level filter"] # [doc = " - `RUST_LOG_STYLE`: whether or not to print styles with records."] # [doc = ""] # [doc = " These sources can be configured using the builder methods on `Env`."] # [derive (Debug)] pub struct Env < 'a > { filter : Var < 'a > , write_style : Var < 'a > , }
    };
}

Env!();