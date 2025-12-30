// Generated macro for CaptureNames (struct)
macro_rules! Depcrate_nfaCaptureNames {
() => {
// Module: crate::nfa
// Provides: {"CaptureNames"}
// Dependencies: {}
# [doc = " An iterator over all capture groups in an NFA."] # [doc = ""] # [doc = " If a particular group has a name, then it is yielded. Otherwise, `None`"] # [doc = " is yielded."] # [derive (Clone , Debug)] pub (crate) struct CaptureNames < 'a > { it : core :: slice :: Iter < 'a , Option < Arc < str > > > , }
};
}
