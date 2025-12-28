macro_rules! deps {
    () => {
        Err!();
        IResult!();
        Needed!();
    };
}

macro_rules! take_until_incomplete_s {
    () => {
        deps!();
        # [test] fn take_until_incomplete_s () { use crate :: bytes :: streaming :: take_until ; fn ys (i : & str) -> IResult < & str , & str > { take_until ("end") (i) } assert_eq ! (ys ("123en") , Err (Err :: Incomplete (Needed :: Unknown))) ; }
    };
}

take_until_incomplete_s!();