macro_rules! deps {
    () => {
        Plot!();
    };
}

macro_rules! Label {
    () => {
        deps!();
        # [doc = " Plot label"] pub struct Label (Cow < 'static , str >) ;
    };
}

Label!()