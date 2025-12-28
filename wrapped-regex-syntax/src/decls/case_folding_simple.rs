macro_rules! case_folding_simple {
    () => {
        # [cfg (feature = "unicode-case")] pub mod case_folding_simple ;
    };
}

case_folding_simple!()