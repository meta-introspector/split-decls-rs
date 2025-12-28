macro_rules! MirSpan {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum MirSpan { ExprId (ExprId) , PatId (PatId) , BindingId (BindingId) , SelfParam , Unknown , }
    };
}

MirSpan!()