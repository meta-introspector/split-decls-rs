macro_rules! deps {
    () => {
        Reference!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use gix_testtools :: size_ok ; use super :: * ; # [test] fn size_of_reference () { let actual = std :: mem :: size_of :: < Reference > () ; let expected = 80 ; assert ! (size_ok (actual , expected) , "let's not let it change size undetected: {actual} <~ {expected}") ; } }
    };
}

tests!();