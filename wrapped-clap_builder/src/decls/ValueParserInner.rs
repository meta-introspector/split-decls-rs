macro_rules! deps {
    () => {
        AnyValueParser!();
    };
}

macro_rules! ValueParserInner {
    () => {
        deps!();
        enum ValueParserInner { Bool , String , OsString , PathBuf , Other (Box < dyn AnyValueParser >) , }
    };
}

ValueParserInner!()