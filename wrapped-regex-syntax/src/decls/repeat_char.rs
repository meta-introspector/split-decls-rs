macro_rules! repeat_char {
    () => {
        fn repeat_char (c : char , count : usize) -> String { core :: iter :: repeat (c) . take (count) . collect () }
    };
}

repeat_char!()