macro_rules! deps {
    () => {
        ExpandError!();
        ExpandResult!();
    };
}

macro_rules! check_tt_count {
    () => {
        deps!();
        fn check_tt_count (tt : & tt :: TopSubtree) -> Result < () , ExpandResult < () > > { let tt = tt . top_subtree () ; let count = tt . count () ; if count <= TOKEN_LIMIT { Ok (()) } else { Err (ExpandResult { value : () , err : Some (ExpandError :: other (tt . delimiter . open , format ! ("macro invocation exceeds token limit: produced {count} tokens, limit is {TOKEN_LIMIT}" ,) ,)) , }) } }
    };
}

check_tt_count!()