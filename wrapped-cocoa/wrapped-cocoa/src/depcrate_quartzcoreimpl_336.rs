// Generated macro for impl_336 (impl)
macro_rules! Depcrate_quartzcoreimpl_336 {
() => {
// Module: crate::quartzcore
// Provides: {"impl_336"}
// Dependencies: {}
impl ContentsGravity { fn into_CFString (self) -> CFString { let string = match self { ContentsGravity :: Center => "center" , ContentsGravity :: Top => "top" , ContentsGravity :: Bottom => "bottom" , ContentsGravity :: Left => "left" , ContentsGravity :: Right => "right" , ContentsGravity :: TopLeft => "topLeft" , ContentsGravity :: TopRight => "topRight" , ContentsGravity :: BottomLeft => "bottomLeft" , ContentsGravity :: BottomRight => "bottomRight" , ContentsGravity :: Resize => "resize" , ContentsGravity :: ResizeAspect => "resizeAspect" , ContentsGravity :: ResizeAspectFill => "resizeAspectFill" , ContentsGravity :: Other (other) => return other , } ; CFString :: from (string) } fn from_CFString (string : CFString) -> ContentsGravity { match string . to_string () { ref s if s == "center" => ContentsGravity :: Center , ref s if s == "top" => ContentsGravity :: Top , ref s if s == "bottom" => ContentsGravity :: Bottom , ref s if s == "left" => ContentsGravity :: Left , ref s if s == "right" => ContentsGravity :: Right , ref s if s == "topLeft" => ContentsGravity :: TopLeft , ref s if s == "topRight" => ContentsGravity :: TopRight , ref s if s == "bottomLeft" => ContentsGravity :: BottomLeft , ref s if s == "bottomRight" => ContentsGravity :: BottomRight , ref s if s == "resize" => ContentsGravity :: Resize , ref s if s == "resizeAspect" => ContentsGravity :: ResizeAspect , ref s if s == "resizeAspectFill" => ContentsGravity :: ResizeAspectFill , _ => ContentsGravity :: Other (string) , } } }
};
}
