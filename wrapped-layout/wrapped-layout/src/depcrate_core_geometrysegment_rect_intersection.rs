// Generated macro for segment_rect_intersection (function)
macro_rules! Depcrate_core_geometrysegment_rect_intersection {
() => {
// Module: crate::core::geometry
// Provides: {"segment_rect_intersection"}
// Dependencies: {}
# [doc = " \\return True if the segment intersects the rect."] pub fn segment_rect_intersection (seg : (Point , Point) , rect : (Point , Point) ,) -> bool { assert ! (rect . 0 . x <= rect . 1 . x) ; assert ! (rect . 0 . y <= rect . 1 . y) ; if seg . 0 . x == seg . 1 . x { return seg . 1 . x >= rect . 0 . x && seg . 1 . x <= rect . 1 . x ; } let above = seg . 0 . x < rect . 0 . x && seg . 1 . x < rect . 0 . x ; let below = seg . 0 . x > rect . 1 . x && seg . 1 . x > rect . 1 . x ; if above || below { return false ; } let above = seg . 0 . y < rect . 0 . y && seg . 1 . y < rect . 0 . y ; let below = seg . 0 . y > rect . 1 . y && seg . 1 . y > rect . 1 . y ; if above || below { return false ; } let dx = seg . 1 . x - seg . 0 . x ; let dy = seg . 1 . y - seg . 0 . y ; let a = dy / dx ; let b = seg . 0 . y - a * seg . 0 . x ; let y0 = a * rect . 0 . x + b ; let y1 = a * rect . 1 . x + b ; let above = y0 < rect . 0 . y && y1 < rect . 0 . y ; let below = y0 > rect . 1 . y && y1 > rect . 1 . y ; ! (above || below) }
};
}
