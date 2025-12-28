macro_rules! Iter {
    () => {
        # [doc = " A special iterator which communicates its operation through results where…"] # [doc = ""] # [doc = " * `Some(Ok(removed_directory))` is yielded once or more success, followed by `None`"] # [doc = " * `Some(Err(std::io::Error))` is yielded exactly once on failure."] pub struct Iter < 'a > { cursor : Option < & 'a Path > , boundary : & 'a Path , }
    };
}

Iter!()