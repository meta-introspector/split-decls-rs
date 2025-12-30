// Generated macro for adjust_by_closest_callsite (function)
macro_rules! Depcrate_copiesadjust_by_closest_callsite {
() => {
// Module: crate::copies
// Provides: {"adjust_by_closest_callsite"}
// Dependencies: {}
# [doc = " Adjusts the index for which the statements begin to differ to the closest macro callsite. This"] # [doc = " avoids giving suggestions that requires splitting a macro call in half, when only a part of the"] # [doc = " macro expansion is equal."] # [doc = ""] # [doc = " For example, for the following macro:"] # [doc = " ```rust,ignore"] # [doc = " macro_rules! foo {"] # [doc = "    ($x:expr) => {"] # [doc = "        let y = 42;"] # [doc = "        $x;"] # [doc = "    };"] # [doc = " }"] # [doc = " ```"] # [doc = " If the macro is called like this:"] # [doc = " ```rust,ignore"] # [doc = " if false {"] # [doc = "    let z = 42;"] # [doc = "    foo!(println!(\"Hello\"));"] # [doc = " } else {"] # [doc = "    let z = 42;"] # [doc = "    foo!(println!(\"World\"));"] # [doc = " }"] # [doc = " ```"] # [doc = " Although the expanded `let y = 42;` is equal, the macro call should not be included in the"] # [doc = " suggestion."] fn adjust_by_closest_callsite < 'tcx > (i : usize , stmt : & 'tcx Stmt < 'tcx > , mut iter : impl Iterator < Item = (usize , & 'tcx Stmt < 'tcx >) > ,) -> usize { let Some ((_ , first)) = iter . next () else { return 0 ; } ; if first . span . source_callsite () != stmt . span . source_callsite () { return i ; } iter . find (| (_ , stmt) | stmt . span . source_callsite () != first . span . source_callsite ()) . map_or (0 , | (i , _) | i + 1) }
};
}
