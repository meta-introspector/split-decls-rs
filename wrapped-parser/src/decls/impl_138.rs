macro_rules! deps {
    () => {
        Pos!();
        Error!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < R : RuleType > From < pest :: error :: Error < R > > for Error { fn from (err : pest :: error :: Error < R >) -> Self { let (start , end) = match err . line_col { LineColLocation :: Pos (at) => (at , None) , LineColLocation :: Span (start , end) => (start , Some (end)) , } ; Error :: Syntax { message : err . to_string () , start : Pos :: from (start) , end : end . map (Pos :: from) , } } }
    };
}

impl_138!()