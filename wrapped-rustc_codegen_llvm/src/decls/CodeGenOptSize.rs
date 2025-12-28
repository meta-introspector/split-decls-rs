macro_rules! CodeGenOptSize {
    () => {
        # [derive (Copy , Clone , PartialEq)] # [repr (C)] pub (crate) enum CodeGenOptSize { CodeGenOptSizeNone = 0 , CodeGenOptSizeDefault = 1 , CodeGenOptSizeAggressive = 2 , }
    };
}

CodeGenOptSize!()