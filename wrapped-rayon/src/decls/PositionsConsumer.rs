macro_rules! PositionsConsumer {
    () => {
        struct PositionsConsumer < 'p , C , P > { base : C , predicate : & 'p P , offset : usize , }
    };
}

PositionsConsumer!();