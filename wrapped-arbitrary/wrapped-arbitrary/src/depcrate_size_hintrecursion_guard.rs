// Generated macro for recursion_guard (function)
macro_rules! Depcrate_size_hintrecursion_guard {
() => {
// Module: crate::size_hint
// Provides: {"recursion_guard"}
// Dependencies: {}
# [doc = " Protects against potential infinite recursion when calculating size hints"] # [doc = " due to indirect type recursion."] # [doc = ""] # [doc = " When the depth is not too deep, calls `f` with `depth + 1` to calculate the"] # [doc = " size hint."] # [doc = ""] # [doc = " Otherwise, returns the default size hint: `(0, None)`."] # [doc = ""] # [doc = " <div class=\"warning\">This method is deprecated. Users should instead implement <a href=\"../trait.Arbitrary.html#method.try_size_hint\"><code>try_size_hint</code></a> and use <a href=\"fn.try_recursion_guard.html\"><code>try_recursion_guard</code></a></div>"] # [inline] # [deprecated (note = "use `try_recursion_guard` instead")] pub fn recursion_guard (depth : usize , f : impl FnOnce (usize) -> (usize , Option < usize >) ,) -> (usize , Option < usize >) { if depth > MAX_DEPTH { (0 , None) } else { f (depth + 1) } }
};
}
