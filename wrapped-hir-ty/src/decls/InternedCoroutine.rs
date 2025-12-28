macro_rules! InternedCoroutine {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct InternedCoroutine (pub DefWithBodyId , pub ExprId) ;
    };
}

InternedCoroutine!();