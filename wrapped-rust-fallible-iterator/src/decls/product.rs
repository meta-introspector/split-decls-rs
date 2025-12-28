macro_rules! product {
    () => {
        # [test] fn product () { assert_eq ! (convert ([1 , 2 , 3 , 4] . iter () . map (Ok ::<& u32 , () >)) . product ::< u32 > () . unwrap () , 24) }
    };
}

product!();