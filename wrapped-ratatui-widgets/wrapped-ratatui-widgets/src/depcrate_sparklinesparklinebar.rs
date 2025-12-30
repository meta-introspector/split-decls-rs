// Generated macro for SparklineBar (struct)
macro_rules! Depcrate_sparklineSparklineBar {
() => {
// Module: crate::sparkline
// Provides: {"SparklineBar"}
// Dependencies: {}
# [doc = " An bar in a `Sparkline`."] # [doc = ""] # [doc = " The height of the bar is determined by the value and a value of `None` is interpreted as the"] # [doc = " _absence_ of a value, as distinct from a value of `Some(0)`."] # [derive (Debug , Default , Copy , Clone , Eq , PartialEq)] pub struct SparklineBar { # [doc = " The value of the bar."] # [doc = ""] # [doc = " If `None`, the bar is absent."] value : Option < u64 > , # [doc = " The style of the bar."] # [doc = ""] # [doc = " If `None`, the bar will use the style of the sparkline."] style : Option < Style > , }
};
}
