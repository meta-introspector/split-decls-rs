macro_rules! InlayKind {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum InlayKind { Adjustment , BindingMode , Chaining , ClosingBrace , ClosureCapture , Discriminant , GenericParamList , Lifetime , Parameter , GenericParameter , Type , Dyn , Drop , RangeExclusive , ExternUnsafety , }
    };
}

InlayKind!()