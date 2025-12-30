// Generated macro for ArrowShape (enum)
macro_rules! DepcrateArrowShape {
() => {
// Module: crate
// Provides: {"ArrowShape"}
// Dependencies: {}
# [doc = " This enumeration represents all possible arrow edge"] # [doc = " as defined in [graphviz documentation](https://graphviz.org/doc/info/arrows.html)."] # [derive (Clone , Copy , Hash , PartialEq , Eq)] pub enum ArrowShape { # [doc = " No arrow will be displayed"] NoArrow , # [doc = " Arrow that ends in a triangle. Basically a normal arrow."] # [doc = " NOTE: there is error in official documentation, this supports both fill and side clipping"] Normal (Fill , Side) , # [doc = " Arrow ending in a small square box"] Box (Fill , Side) , # [doc = " Arrow ending in a three branching lines also called crow's foot"] Crow (Side) , # [doc = " Arrow ending in a curve"] Curve (Side) , # [doc = " Arrow ending in an inverted curve"] ICurve (Fill , Side) , # [doc = " Arrow ending in a diamond shaped rectangular shape."] Diamond (Fill , Side) , # [doc = " Arrow ending in a circle."] Dot (Fill) , # [doc = " Arrow ending in an inverted triangle."] Inv (Fill , Side) , # [doc = " Arrow ending with a T shaped arrow."] Tee (Side) , # [doc = " Arrow ending with a V shaped arrow."] Vee (Side) , }
};
}
