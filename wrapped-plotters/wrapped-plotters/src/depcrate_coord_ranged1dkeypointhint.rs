// Generated macro for KeyPointHint (trait)
macro_rules! Depcrate_coord_ranged1dKeyPointHint {
() => {
// Module: crate::coord::ranged1d
// Provides: {"KeyPointHint"}
// Dependencies: {}
# [doc = " The trait for a hint provided to the key point algorithm used by the coordinate specs."] # [doc = " The most important constraint is the `max_num_points` which means the algorithm could emit no more than specific number of key points"] # [doc = " `weight` is used to determine if this is used as a bold grid line or light grid line"] # [doc = " `bold_points` returns the max number of corresponding bold grid lines"] pub trait KeyPointHint { # [doc = " Returns the max number of key points"] fn max_num_points (& self) -> usize ; # [doc = " Returns the weight for this hint"] fn weight (& self) -> KeyPointWeight ; # [doc = " Returns the point number constraint for the bold points"] fn bold_points (& self) -> usize { self . max_num_points () } }
};
}
