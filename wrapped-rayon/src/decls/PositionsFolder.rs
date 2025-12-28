macro_rules! PositionsFolder {
    () => {
        struct PositionsFolder < 'p , F , P > { base : F , predicate : & 'p P , offset : usize , }
    };
}

PositionsFolder!()