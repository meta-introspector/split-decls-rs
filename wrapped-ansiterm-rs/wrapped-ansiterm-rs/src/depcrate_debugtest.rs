// Generated macro for test (module)
macro_rules! Depcrate_debugtest {
() => {
// Module: crate::debug
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: style :: Colour :: * ; use crate :: style :: Style ; fn style () -> Style { Style :: new () } macro_rules ! test { ($ name : ident : $ obj : expr => $ result : expr) => { # [test] fn $ name () { assert_eq ! ($ result , format ! ("{:?}" , $ obj)) ; } } ; } test ! (empty : style () => "Style {}") ; test ! (bold : style () . bold () => "Style { bold }") ; test ! (italic : style () . italic () => "Style { italic }") ; test ! (both : style () . bold () . italic () => "Style { bold, italic }") ; test ! (red : Red . normal () => "Style { fg(Red) }") ; test ! (redblue : Red . normal () . on (RGB (3 , 2 , 4)) => "Style { fg(Red), on(RGB(3, 2, 4)) }") ; test ! (everything : Red . on (Blue) . blink () . bold () . dimmed () . hidden () . italic () . reverse () . strikethrough () . underline () => "Style { fg(Red), on(Blue), blink, bold, dimmed, hidden, italic, reverse, strikethrough, underline }") ; # [test] fn long_and_detailed () { let expected_debug = "Style { fg(Blue), bold }" ; let expected_pretty_repat = r##"(?x)
        Style\s+\{\s+
            foreground:\s+Some\(\s+
                Blue,?\s+
            \),\s+
            background:\s+None,\s+
            blink:\s+false,\s+
            bold:\s+true,\s+
            dimmed:\s+false,\s+
            hidden:\s+false,\s+
            italic:\s+false,\s+
            reverse:\s+false,\s+
            strikethrough:\s+
            false,\s+
            underline:\s+false,?\s+
            \}"## ; let re = regex :: Regex :: new (expected_pretty_repat) . unwrap () ; let style = Blue . bold () ; let style_fmt_debug = format ! ("{:?}" , style) ; let style_fmt_pretty = format ! ("{:#?}" , style) ; println ! ("style_fmt_debug:\n{}" , style_fmt_debug) ; println ! ("style_fmt_pretty:\n{}" , style_fmt_pretty) ; assert_eq ! (expected_debug , style_fmt_debug) ; assert ! (re . is_match (& style_fmt_pretty)) ; } }
};
}
