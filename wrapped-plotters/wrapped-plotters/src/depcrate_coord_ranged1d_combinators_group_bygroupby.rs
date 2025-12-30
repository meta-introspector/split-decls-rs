// Generated macro for GroupBy (struct)
macro_rules! Depcrate_coord_ranged1d_combinators_group_byGroupBy {
() => {
// Module: crate::coord::ranged1d::combinators::group_by
// Provides: {"GroupBy"}
// Dependencies: {}
# [doc = " Grouping the value in the coordinate specification."] # [doc = ""] # [doc = " This combinator doesn't change the coordinate mapping behavior. But it changes how"] # [doc = " the key point is generated, this coordinate specification will enforce that only the first value in each group"] # [doc = " can be emitted as the bold key points."] # [doc = ""] # [doc = " This is useful, for example, when we have an X axis is a integer and denotes days."] # [doc = " And we are expecting the tick mark denotes weeks, in this way we can make the range"] # [doc = " spec grouping by 7 elements."] # [doc = " With the help of the GroupBy decorator, this can be archived quite easily:"] # [doc = "```rust"] # [doc = "use plotters::prelude::*;"] # [doc = "let mut buf = vec![0;1024*768*3];"] # [doc = "let area = BitMapBackend::with_buffer(buf.as_mut(), (1024, 768)).into_drawing_area();"] # [doc = "let chart = ChartBuilder::on(&area)"] # [doc = "    .build_cartesian_2d((0..100).group_by(7), 0..100)"] # [doc = "    .unwrap();"] # [doc = "```"] # [doc = ""] # [doc = " To apply this combinator, call [ToGroupByRange::group_by](trait.ToGroupByRange.html#tymethod.group_by) method on any discrete coordinate spec."] # [derive (Clone)] pub struct GroupBy < T : DiscreteRanged > (T , usize) ;
};
}
