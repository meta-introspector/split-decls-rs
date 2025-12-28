macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! resumable_extend {
    () => {
        deps!();
        # [test] fn resumable_extend () { let s = "a b c" ; let it = s . chars () . scan (0 , | _ , ch | if ch . is_whitespace () { None } else { Some (ch) }) ; let mut v : SmallVec < char , 4 > = SmallVec :: new () ; v . extend (it) ; assert_eq ! (v [..] , ['a']) ; }
    };
}

resumable_extend!()