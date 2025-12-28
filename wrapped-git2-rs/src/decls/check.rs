macro_rules! check {
    () => {
        pub fn check () { let err = LAST_ERROR . with (| slot | slot . borrow_mut () . take ()) ; if let Some (err) = err { std :: panic :: resume_unwind (err) ; } }
    };
}

check!();