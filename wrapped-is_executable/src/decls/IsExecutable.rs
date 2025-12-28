macro_rules! IsExecutable {
    () => {
        # [doc = " An extension trait for `std::fs::Path` providing an `is_executable` method."] # [doc = ""] # [doc = " See the module documentation for examples."] pub trait IsExecutable { # [doc = " Returns `true` if there is a file at the given path and it is"] # [doc = " executable. Returns `false` otherwise."] # [doc = ""] # [doc = " See the module documentation for details."] fn is_executable (& self) -> bool ; }
    };
}

IsExecutable!()