// Generated macro for Strategy (enum)
macro_rules! Depcrate_astStrategy {
() => {
// Module: crate::ast
// Provides: {"Strategy"}
// Dependencies: {}
# [doc = " The type of a given `Strategy`."] pub enum Strategy { # [doc = " Assuming the metavariable `$ty` for a given type, this models the"] # [doc = " strategy type `<$ty as Arbitrary>::Strategy`."] Arbitrary (syn :: Type , Span) , # [doc = " This models <$ty as StrategyFromRegex>::Strategy."] Regex (syn :: Type) , # [doc = " Assuming the metavariable `$ty` for a given type, this models the"] # [doc = " strategy type `BoxedStrategy<$ty>`, i.e: an existentially typed strategy."] # [doc = ""] # [doc = " The dynamic dispatch used here is an implementation detail that may be"] # [doc = " changed. Such a change does not count as a breakage semver wise."] Existential (syn :: Type) , # [doc = " Assuming the metavariable `$ty` for a given type, this models a"] # [doc = " non-shrinking strategy that simply always returns a value of the"] # [doc = " given type."] Value (syn :: Type) , # [doc = " Assuming a sequence of strategies, this models a mapping from that"] # [doc = " sequence to `Self`."] Map (Box < [Strategy] >) , # [doc = " Assuming a sequence of relative-weighted strategies, this models a"] # [doc = " weighted choice of those strategies. The resultant strategy will in"] # [doc = " other words randomly pick one strategy with probabilities based on the"] # [doc = " specified weights."] Union (Box < [Strategy] >) , # [doc = " A filtered strategy with `.prop_filter`."] Filter (Box < Strategy > , syn :: Type) , }
};
}
