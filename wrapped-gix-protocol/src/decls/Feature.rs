macro_rules! Feature {
    () => {
        # [doc = " A key value pair of values known at compile time."] pub type Feature = (& 'static str , Option < Cow < 'static , str > >) ;
    };
}

Feature!();