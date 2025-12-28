macro_rules! BlockInfo {
    () => {
        # [derive (Clone)] struct BlockInfo { name : String , spans : Vec < Span > , suggs : Vec < Span > , }
    };
}

BlockInfo!();