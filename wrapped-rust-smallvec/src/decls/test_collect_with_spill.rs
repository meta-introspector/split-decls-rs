macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_collect_with_spill {
    () => {
        deps!();
        # [test] fn test_collect_with_spill () { let input = "0123456" ; let collected : SmallVec < char , 4 > = input . chars () . collect () ; assert_eq ! (collected , & ['0' , '1' , '2' , '3' , '4' , '5' , '6']) ; }
    };
}

test_collect_with_spill!();