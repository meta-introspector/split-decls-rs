// Generated macro for Position (enum)
macro_rules! Depcrate_keyPosition {
() => {
// Module: crate::key
// Provides: {"Position"}
// Dependencies: {}
# [doc = " Position of the key"] # [derive (Clone , Copy)] pub enum Position { # [doc = " Inside the area surrounded by the four (Bottom X, Top X, Left Y and Right Y) axes"] Inside (Vertical , Horizontal) , # [doc = " Outside of that area"] Outside (Vertical , Horizontal) , }
};
}
