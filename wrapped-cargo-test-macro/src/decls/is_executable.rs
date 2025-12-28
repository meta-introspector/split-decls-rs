macro_rules! is_executable {
    () => {
        # [cfg (windows)] fn is_executable < P : AsRef < Path > > (path : P) -> bool { path . as_ref () . is_file () }
    };
}

is_executable!();