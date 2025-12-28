macro_rules! deps {
    () => {
        CvQualifiers!();
    };
}

macro_rules! FunctionParam {
    () => {
        deps!();
        # [doc = " The <function-param> production."] # [doc = ""] # [doc = " ```text"] # [doc = " <function-param> ::= fp <top-level CV-qualifiers> _"] # [doc = "                          # L == 0, first parameter"] # [doc = "                  ::= fp <top-level CV-qualifiers> <parameter-2 non-negative number> _"] # [doc = "                          # L == 0, second and later parameters"] # [doc = "                  ::= fL <L-1 non-negative number> p <top-level CV-qualifiers> _"] # [doc = "                          # L > 0, first parameter"] # [doc = "                  ::= fL <L-1 non-negative number> p <top-level CV-qualifiers> <parameter-2 non-negative number> _"] # [doc = "                          # L > 0, second and later parameters"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct FunctionParam (usize , CvQualifiers , Option < usize >) ;
    };
}

FunctionParam!();