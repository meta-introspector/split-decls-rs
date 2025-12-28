macro_rules! update_expect {
    () => {
        fn update_expect () -> bool { env :: var ("UPDATE_EXPECT") . is_ok () }
    };
}

update_expect!()