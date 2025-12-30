// Generated macro for Union (struct)
macro_rules! Depcrate_strategy_unionsUnion {
() => {
// Module: crate::strategy::unions
// Provides: {"Union"}
// Dependencies: {}
# [doc = " A `Strategy` which picks from one of several delegate `Strategy`s."] # [doc = ""] # [doc = " See `Strategy::prop_union()`."] # [derive (Clone , Debug)] # [must_use = "strategies do nothing unless used"] pub struct Union < T : Strategy > { options : Vec < WA < T > > , }
};
}
