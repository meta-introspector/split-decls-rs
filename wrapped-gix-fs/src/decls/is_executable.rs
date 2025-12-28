macro_rules! is_executable {
    () => {
        # [cfg (not (unix))] # [doc = " Returns whether a file has the executable permission set."] pub fn is_executable (_metadata : & std :: fs :: Metadata) -> bool { false }
    };
}

is_executable!();