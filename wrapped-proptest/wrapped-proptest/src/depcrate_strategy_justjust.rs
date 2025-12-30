// Generated macro for Just (struct)
macro_rules! Depcrate_strategy_justJust {
() => {
// Module: crate::strategy::just
// Provides: {"Just"}
// Dependencies: {}
# [doc = " A `Strategy` which always produces a single value value and never"] # [doc = " simplifies."] # [derive (Clone , Copy , Debug)] # [must_use = "strategies do nothing unless used"] pub struct Just < T : Clone + fmt :: Debug > (# [doc = " The value produced by this strategy."] pub T ,) ;
};
}
