macro_rules! DebugArgPath {
    () => {
        struct DebugArgPath < 'a > (pub & 'a Path) ;
    };
}

DebugArgPath!();