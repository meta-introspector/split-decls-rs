// Generated macro for test_markup (function)
macro_rules! Depcrate_features_markuptest_markup {
() => {
// Module: crate::features::markup
// Provides: {"test_markup"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_markup () { const TEXT : & str = "<script>alert('Hello, world!')</script>" ; markup :: define ! (Template < M : Render > (msg : M) { textarea { @ msg } }) ; let compact = Template { msg : CompactString :: from (TEXT) , } ; let control = Template { msg : String :: from (TEXT) , } ; assert_eq ! (compact . to_string () , "<textarea>&lt;script&gt;alert('Hello, world!')&lt;/script&gt;</textarea>" ,) ; assert_eq ! (control . to_string () , "<textarea>&lt;script&gt;alert('Hello, world!')&lt;/script&gt;</textarea>" ,) ; }
};
}
