macro_rules! try_canonicalize {
    () => {
        # [inline] pub fn try_canonicalize < P : AsRef < Path > > (path : P) -> io :: Result < PathBuf > { fs :: canonicalize (& path) . or_else (| _ | absolute (& path)) }
    };
}

try_canonicalize!();