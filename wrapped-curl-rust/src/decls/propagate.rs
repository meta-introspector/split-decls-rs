macro_rules! propagate {
    () => {
        pub fn propagate () { if let Ok (Some (t)) = LAST_ERROR . try_with (| slot | slot . borrow_mut () . take ()) { panic :: resume_unwind (t) } }
    };
}

propagate!();