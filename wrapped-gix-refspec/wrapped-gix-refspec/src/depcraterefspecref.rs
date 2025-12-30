// Generated macro for RefSpecRef (struct)
macro_rules! DepcrateRefSpecRef {
() => {
// Module: crate
// Provides: {"RefSpecRef"}
// Dependencies: {}
# [doc = " A refspec with references to the memory it was parsed from."] # [derive (Eq , Copy , Clone , Debug)] pub struct RefSpecRef < 'a > { mode : types :: Mode , op : parse :: Operation , src : Option < & 'a bstr :: BStr > , dst : Option < & 'a bstr :: BStr > , }
};
}
