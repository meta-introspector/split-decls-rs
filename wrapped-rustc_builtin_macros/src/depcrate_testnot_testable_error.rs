// Generated macro for not_testable_error (function)
macro_rules! Depcrate_testnot_testable_error {
() => {
// Module: crate::test
// Provides: {"not_testable_error"}
// Dependencies: {}
fn not_testable_error (cx : & ExtCtxt < '_ > , attr_sp : Span , item : Option < & ast :: Item >) { let dcx = cx . dcx () ; let msg = "the `#[test]` attribute may only be used on a non-associated function" ; let level = match item . map (| i | & i . kind) { Some (ast :: ItemKind :: MacCall (_)) => Level :: Warning , _ => Level :: Error , } ; let mut err = Diag :: < () > :: new (dcx , level , msg) ; err . span (attr_sp) ; if let Some (item) = item { err . span_label (item . span , format ! ("expected a non-associated function, found {} {}" , item . kind . article () , item . kind . descr ()) ,) ; } err . with_span_label (attr_sp , "the `#[test]` macro causes a function to be run as a test and has no effect on non-functions") . with_span_suggestion (attr_sp , "replace with conditional compilation to make the item only exist when tests are being run" , "#[cfg(test)]" , Applicability :: MaybeIncorrect) . emit () ; }
};
}
