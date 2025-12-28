macro_rules! EarlyLintPassObject {
    () => {
        # [doc = " A lint pass boxed up as a trait object."] pub (crate) type EarlyLintPassObject = Box < dyn EarlyLintPass + 'static > ;
    };
}

EarlyLintPassObject!()