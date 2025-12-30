// Generated macro for MatchWords (type)
macro_rules! DepcrateMatchWords {
() => {
// Module: crate
// Provides: {"MatchWords"}
// Dependencies: {}
type MatchWords < 'a , B > = Chain < Enumerate < Blocks < 'a , B > > , Skip < Take < Enumerate < Repeat < B > > > > > ;
};
}
