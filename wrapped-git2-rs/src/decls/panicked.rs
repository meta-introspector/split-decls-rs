macro_rules! panicked {
    () => {
        pub fn panicked () -> bool { LAST_ERROR . with (| slot | slot . borrow () . is_some ()) }
    };
}

panicked!()