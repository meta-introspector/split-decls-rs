macro_rules! MismatchedLifetimeSyntaxesSuggestion {
    () => {
        # [derive (Debug)] pub (crate) enum MismatchedLifetimeSyntaxesSuggestion { Implicit { suggestions : Vec < Span > , optional_alternative : bool , } , Mixed { implicit_suggestions : Vec < Span > , explicit_anonymous_suggestions : Vec < (Span , String) > , optional_alternative : bool , } , Explicit { lifetime_name : String , suggestions : Vec < (Span , String) > , optional_alternative : bool , } , }
    };
}

MismatchedLifetimeSyntaxesSuggestion!()