macro_rules! deps {
    () => {
        Mode!();
    };
}

macro_rules! is_symlink {
    () => {
        deps!();
        fn is_symlink (mode : Option < component :: Mode >) -> bool { mode == Some (component :: Mode :: Symlink) }
    };
}

is_symlink!();