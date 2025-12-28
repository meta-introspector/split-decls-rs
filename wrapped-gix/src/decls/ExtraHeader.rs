macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! ExtraHeader {
    () => {
        deps!();
        # [doc = " The `http.extraHeader` key."] pub type ExtraHeader = keys :: Any < validate :: ExtraHeader > ;
    };
}

ExtraHeader!()