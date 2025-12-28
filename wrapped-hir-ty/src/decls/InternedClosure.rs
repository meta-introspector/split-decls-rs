macro_rules! InternedClosure {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct InternedClosure (pub DefWithBodyId , pub ExprId) ;
    };
}

InternedClosure!();