macro_rules! HygieneId {
    () => {
        # [doc = " A wrapper around [`span::SyntaxContextId`] that is intended only for comparisons."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct HygieneId (span :: SyntaxContext) ;
    };
}

HygieneId!();