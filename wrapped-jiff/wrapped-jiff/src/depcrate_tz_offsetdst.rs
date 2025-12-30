// Generated macro for Dst (enum)
macro_rules! Depcrate_tz_offsetDst {
() => {
// Module: crate::tz::offset
// Provides: {"Dst"}
// Dependencies: {}
# [doc = " An enum indicating whether a particular datetime  is in DST or not."] # [doc = ""] # [doc = " DST stands for \"daylight saving time.\" It is a label used to apply to"] # [doc = " points in time as a way to contrast it with \"standard time.\" DST is"] # [doc = " usually, but not always, one hour ahead of standard time. When DST takes"] # [doc = " effect is usually determined by governments, and the rules can vary"] # [doc = " depending on the location. DST is typically used as a means to maximize"] # [doc = " \"sunlight\" time during typical working hours, and as a cost cutting measure"] # [doc = " by reducing energy consumption. (The effectiveness of DST and whether it"] # [doc = " is overall worth it is a separate question entirely.)"] # [doc = ""] # [doc = " In general, most users should never need to deal with this type. But it can"] # [doc = " be occasionally useful in circumstances where callers need to know whether"] # [doc = " DST is active or not for a particular point in time."] # [doc = ""] # [doc = " This type has a `From<bool>` trait implementation, where the bool is"] # [doc = " interpreted as being `true` when DST is active."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] pub enum Dst { # [doc = " DST is not in effect. In other words, standard time is in effect."] No , # [doc = " DST is in effect."] Yes , }
};
}
