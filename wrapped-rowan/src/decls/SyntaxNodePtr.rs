macro_rules! deps {
    () => {
        Language!();
        SyntaxNode!();
    };
}

macro_rules! SyntaxNodePtr {
    () => {
        deps!();
        # [doc = " A \"pointer\" to a [`SyntaxNode`], via location in the source code."] # [doc = ""] # [doc = " ## Note"] # [doc = " Since the location is source code dependent, this must not be used"] # [doc = " with mutable syntax trees. Any changes made in such trees causes"] # [doc = " the pointed node's source location to change, invalidating the pointer."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub struct SyntaxNodePtr < L : Language > { kind : L :: Kind , range : TextRange , }
    };
}

SyntaxNodePtr!()