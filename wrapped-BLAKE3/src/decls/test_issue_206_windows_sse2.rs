macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! test_issue_206_windows_sse2 {
    () => {
        deps!();
        # [test] fn test_issue_206_windows_sse2 () { for _ in & [0] { let input = & [0xff ; 65] ; let expected_hash = [183 , 235 , 50 , 217 , 156 , 24 , 190 , 219 , 2 , 216 , 176 , 255 , 224 , 53 , 28 , 95 , 57 , 148 , 179 , 245 , 162 , 90 , 37 , 121 , 0 , 142 , 219 , 62 , 234 , 204 , 225 , 161 ,] ; crate :: Hasher :: new () . update (input) ; assert_eq ! (crate :: Hasher :: new () . update (input) . finalize () , expected_hash) ; } }
    };
}

test_issue_206_windows_sse2!()