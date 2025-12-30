// Generated macro for centered_rect (function)
macro_rules! Depcrate_destroycentered_rect {
() => {
// Module: crate::destroy
// Provides: {"centered_rect"}
// Dependencies: {}
# [doc = " a centered rect of the given size"] fn centered_rect (area : Rect , width : u16 , height : u16) -> Rect { let horizontal = Layout :: horizontal ([width]) . flex (Flex :: Center) ; let vertical = Layout :: vertical ([height]) . flex (Flex :: Center) ; let [area] = area . layout (& vertical) ; let [area] = area . layout (& horizontal) ; area }
};
}
