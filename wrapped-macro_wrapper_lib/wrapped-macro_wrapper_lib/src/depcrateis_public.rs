// Generated macro for is_public (function)
macro_rules! Depcrateis_public {
() => {
// Module: crate
// Provides: {"is_public"}
// Dependencies: {}
# [doc = " Helper to determine if an item is public"] fn is_public (vis : & Visibility) -> bool { matches ! (vis , Visibility :: Public (_)) }
};
}
