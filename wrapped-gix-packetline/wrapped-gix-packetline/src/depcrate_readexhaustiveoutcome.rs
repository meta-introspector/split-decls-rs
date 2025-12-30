// Generated macro for ExhaustiveOutcome (type)
macro_rules! Depcrate_readExhaustiveOutcome {
() => {
// Module: crate::read
// Provides: {"ExhaustiveOutcome"}
// Dependencies: {}
# [cfg (any (feature = "blocking-io" , feature = "async-io"))] pub (crate) type ExhaustiveOutcome < 'a > = (bool , Option < PacketLineRef < 'static > > , Option < std :: io :: Result < Result < PacketLineRef < 'a > , crate :: decode :: Error > > > ,) ;
};
}
