macro_rules! visualize_whitespace {
    () => {
        fn visualize_whitespace (input : & str) -> String { input . to_owned () . replace ('\r' , "␍") . replace ('\n' , "␊") }
    };
}

visualize_whitespace!()