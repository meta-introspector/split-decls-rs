macro_rules! deps {
    () => {
        ChangeListRef!();
        ConflictMapping!();
    };
}

macro_rules! pick_our_changes {
    () => {
        deps!();
        fn pick_our_changes < 'a > (side : ConflictMapping , ours : & 'a ChangeListRef , theirs : & 'a ChangeListRef ,) -> & 'a ChangeListRef { match side { Original => ours , Swapped => theirs , } }
    };
}

pick_our_changes!()