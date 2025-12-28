macro_rules! deps {
    () => {
        ConflictMapping!();
        ChangeList!();
    };
}

macro_rules! pick_our_changes_mut {
    () => {
        deps!();
        fn pick_our_changes_mut < 'a > (side : ConflictMapping , ours : & 'a mut ChangeList , theirs : & 'a mut ChangeList ,) -> & 'a mut ChangeList { match side { Original => ours , Swapped => theirs , } }
    };
}

pick_our_changes_mut!()