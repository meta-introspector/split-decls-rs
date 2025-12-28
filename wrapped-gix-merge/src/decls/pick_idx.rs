macro_rules! deps {
    () => {
        ConflictMapping!();
    };
}

macro_rules! pick_idx {
    () => {
        deps!();
        fn pick_idx (side : ConflictMapping , ours : usize , theirs : usize) -> usize { match side { Original => ours , Swapped => theirs , } }
    };
}

pick_idx!();