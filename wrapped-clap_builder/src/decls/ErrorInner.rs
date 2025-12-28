macro_rules! deps {
    () => {
        Message!();
        ContextValue!();
        Backtrace!();
        FlatMap!();
        ContextKind!();
        ColorChoice!();
        Error!();
        ErrorKind!();
        Styles!();
    };
}

macro_rules! ErrorInner {
    () => {
        deps!();
        # [derive (Debug)] struct ErrorInner { kind : ErrorKind , # [cfg (feature = "error-context")] context : FlatMap < ContextKind , ContextValue > , message : Option < Message > , source : Option < Box < dyn error :: Error + Send + Sync > > , help_flag : Option < Cow < 'static , str > > , styles : Styles , color_when : ColorChoice , color_help_when : ColorChoice , backtrace : Option < Backtrace > , }
    };
}

ErrorInner!();