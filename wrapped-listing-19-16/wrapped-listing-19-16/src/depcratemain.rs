// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let msg = Message :: ChangeColor (Color :: Hsv (0 , 160 , 255)) ; match msg { Message :: ChangeColor (Color :: Rgb (r , g , b)) => { println ! ("Change color to red {r}, green {g}, and blue {b}") ; } Message :: ChangeColor (Color :: Hsv (h , s , v)) => { println ! ("Change color to hue {h}, saturation {s}, value {v}") ; } _ => () , } }
};
}
