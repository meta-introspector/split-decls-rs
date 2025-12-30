// Generated macro for get_shape_size (function)
macro_rules! Depcrate_std_shapes_renderget_shape_size {
() => {
// Module: crate::std_shapes::render
// Provides: {"get_shape_size"}
// Dependencies: {}
# [doc = " Return the size of the shape. If \\p make_xy_same is set then make the"] # [doc = " X and the Y of the shape the same. This will turn ellipses into circles and"] # [doc = " rectangles into boxes. The parameter \\p dir specifies the direction of the"] # [doc = " graph. This tells us if we need to draw records left to right or top down."] pub fn get_shape_size (dir : Orientation , s : & ShapeKind , font : usize , make_xy_same : bool ,) -> Point { let mut res = match s { ShapeKind :: Box (text) => { pad_shape_scalar (get_size_for_str (text , font) , BOX_SHAPE_PADDING) } ShapeKind :: Circle (text) => { pad_shape_scalar (get_size_for_str (text , font) , CIRCLE_SHAPE_PADDING) } ShapeKind :: DoubleCircle (text) => { pad_shape_scalar (get_size_for_str (text , font) , CIRCLE_SHAPE_PADDING) } ShapeKind :: Record (sr) => { pad_shape_scalar (get_record_size (sr , dir , font) , BOX_SHAPE_PADDING) } ShapeKind :: Connector (text) => { if let Option :: Some (text) = text { pad_shape_scalar (get_size_for_str (text , font) , BOX_SHAPE_PADDING ,) } else { Point :: new (1. , 1.) } } _ => Point :: new (1. , 1.) , } ; if make_xy_same { res = make_size_square (res) ; } res }
};
}
