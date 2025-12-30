// Generated macro for Algorithm (enum)
macro_rules! DepcrateAlgorithm {
() => {
// Module: crate
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " The way the negotiation is performed."] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq)] pub enum Algorithm { # [doc = " Do not send any information at all, which typically leads to complete packs to be sent."] Noop , # [doc = " Walk over consecutive commits and check each one. This can be costly be assures packs are exactly the size they need to be."] # [default] Consecutive , # [doc = " Like `Consecutive`, but skips commits to converge faster, at the cost of receiving packs that are larger than they have to be."] Skipping , }
};
}
