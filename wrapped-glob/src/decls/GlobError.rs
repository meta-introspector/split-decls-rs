macro_rules! GlobError {
    () => {
        # [doc = " A glob iteration error."] # [doc = ""] # [doc = " This is typically returned when a particular path cannot be read"] # [doc = " to determine if its contents match the glob pattern. This is possible"] # [doc = " if the program lacks the appropriate permissions, for example."] # [derive (Debug)] pub struct GlobError { path : PathBuf , error : io :: Error , }
    };
}

GlobError!()