// Generated macro for PathPrinter (struct)
macro_rules! Depcrate_pathPathPrinter {
() => {
// Module: crate::path
// Provides: {"PathPrinter"}
// Dependencies: {}
# [doc = " A printer file paths, with optional color and hyperlink support."] # [doc = ""] # [doc = " This printer is very similar to [`Summary`](crate::Summary) in that it"] # [doc = " principally only emits file paths. The main difference is that this printer"] # [doc = " doesn't actually execute any search via a `Sink` implementation, and instead"] # [doc = " just provides a way for the caller to print paths."] # [doc = ""] # [doc = " A caller could just print the paths themselves, but this printer handles"] # [doc = " a few details:"] # [doc = ""] # [doc = " * It can normalize path separators."] # [doc = " * It permits configuring the terminator."] # [doc = " * It allows setting the color configuration in a way that is consistent"] # [doc = " with the other printers in this crate."] # [doc = " * It allows setting the hyperlink format in a way that is consistent"] # [doc = " with the other printers in this crate."] # [derive (Debug)] pub struct PathPrinter < W > { config : Config , wtr : W , interpolator : hyperlink :: Interpolator , }
};
}
