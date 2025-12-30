// Generated macro for Position (struct)
macro_rules! Depcrate_core_geometryPosition {
() => {
// Module: crate::core::geometry
// Provides: {"Position"}
// Dependencies: {}
# [doc = " Represents the size, location and centerpoint of a shape. We align shapes"] # [doc = " along their center points, and have edges directed at the center. Shapes"] # [doc = " like Box and Circle have their center point in the middle, but labels have"] # [doc = " their center point in one of the sides to make sure that edges don't"] # [doc = " obscure the text. The halo is the gap around the shape where nothing can be"] # [doc = " placed and it is applied symmetrically to the sides."] # [doc = ""] # [doc = " This struct has fields that represent the following points:"] # [doc = "   ____________________"] # [doc = "  |    _____________   |"] # [doc = "  |  |             |   |"] # [doc = "  |  |             |   |"] # [doc = "  |  |      M <----|---|--the middle of the shape, in absolute coordinates."] # [doc = "  |  |         C <-|---|--the center point, saved as delta, relative to M."] # [doc = "  |  |_____________|   |"] # [doc = "  |                ^---|--- the size of the shape."] # [doc = "  |____________________| <----- size + halo."] # [doc = ""] # [derive (Debug , Clone , Copy)] pub struct Position { middle : Point , size : Point , center : Point , halo : Point , }
};
}
