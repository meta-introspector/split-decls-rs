macro_rules! DiagArgName {
    () => {
        # [doc = " Name of a diagnostic argument."] pub type DiagArgName = Cow < 'static , str > ;
    };
}

DiagArgName!()