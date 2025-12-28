macro_rules! check {
    () => {
        fn check (def_edition : Edition , call_edition : Edition , decl : & str , arg : & str , expect : expect_test :: Expect ,) { check_ (def_edition , call_edition , false , decl , arg , true , expect , parser :: TopEntryPoint :: SourceFile ,) ; }
    };
}

check!()