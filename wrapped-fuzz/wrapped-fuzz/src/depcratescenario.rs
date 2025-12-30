// Generated macro for Scenario (struct)
macro_rules! DepcrateScenario {
() => {
// Module: crate
// Provides: {"Scenario"}
// Dependencies: {}
# [doc = " A framework to generate a `CompactString` and control `String`, and then run a series of actions"] # [doc = " and assert equality"] # [doc = ""] # [doc = " Used for fuzz testing"] # [derive (Arbitrary , Debug)] pub struct Scenario < 'a > { pub creation : Creation < 'a > , pub actions : Vec < Action < 'a > > , pub seed : u64 , }
};
}
