macro_rules! Mark {
    () => {
        struct Mark { start_index : usize , may_collapse : bool , }
    };
}

Mark!();