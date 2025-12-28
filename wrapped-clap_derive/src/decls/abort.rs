macro_rules! abort {
    () => {
        macro_rules ! abort { ($ obj : expr , $ ($ format : tt) +) => { { return Err (format_err ! ($ obj , $ ($ format) +)) ; } } ; }
    };
}

abort!()