macro_rules! deps {
    () => {
        Result!();
        AssistKind!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl FromStr for AssistKind { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "QuickFix" => Ok (AssistKind :: QuickFix) , "Generate" => Ok (AssistKind :: Generate) , "Refactor" => Ok (AssistKind :: Refactor) , "RefactorExtract" => Ok (AssistKind :: RefactorExtract) , "RefactorInline" => Ok (AssistKind :: RefactorInline) , "RefactorRewrite" => Ok (AssistKind :: RefactorRewrite) , unknown => Err (format ! ("Unknown AssistKind: '{unknown}'")) , } } }
    };
}

impl_14!();