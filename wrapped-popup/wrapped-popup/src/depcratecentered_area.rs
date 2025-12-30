// Generated macro for centered_area (function)
macro_rules! Depcratecentered_area {
() => {
// Module: crate
// Provides: {"centered_area"}
// Dependencies: {}
# [doc = " Create a centered rect using up certain percentage of the available rect"] fn centered_area (area : Rect , percent_x : u16 , percent_y : u16) -> Rect { let vertical = Layout :: vertical ([Constraint :: Percentage (percent_y)]) . flex (Flex :: Center) ; let horizontal = Layout :: horizontal ([Constraint :: Percentage (percent_x)]) . flex (Flex :: Center) ; let [area] = area . layout (& vertical) ; let [area] = area . layout (& horizontal) ; area }
};
}
