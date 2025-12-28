macro_rules! assert_ssr_transform {
    () => {
        fn assert_ssr_transform (rule : & str , input : & str , expected : Expect) { assert_ssr_transforms (& [rule] , input , expected) ; }
    };
}

assert_ssr_transform!()