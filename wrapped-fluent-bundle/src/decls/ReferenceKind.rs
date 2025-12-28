macro_rules! ReferenceKind {
    () => {
        # [doc = " Maps an [`InlineExpression`] into the kind of reference, with owned strings"] # [doc = " that identify the expression. This makes it so that the [`InlineExpression`] can"] # [doc = " be used to generate an error string."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum ReferenceKind { Function { id : String , } , Message { id : String , attribute : Option < String > , } , Term { id : String , attribute : Option < String > , } , Variable { id : String , } , }
    };
}

ReferenceKind!()