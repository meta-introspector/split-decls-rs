macro_rules! is_executable {
    () => {
        # [doc = " Returns `true` if there is a file at the given path and it is"] # [doc = " executable. Returns `false` otherwise."] # [doc = ""] # [doc = " See the module documentation for details."] pub fn is_executable < P > (path : P) -> bool where P : AsRef < Path > , { path . as_ref () . is_executable () }
    };
}

is_executable!();