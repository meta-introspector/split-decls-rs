macro_rules! sum {
    () => {
        # [test] fn sum () { assert_eq ! (convert ([0 , 1 , 2 , 3] . iter () . map (Ok ::<& u32 , () >)) . sum ::< u32 > () . unwrap () , 6 ,) ; }
    };
}

sum!();