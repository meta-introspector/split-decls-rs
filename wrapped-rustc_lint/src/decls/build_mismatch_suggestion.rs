macro_rules! deps {
    () => {
        Info!();
        MismatchedLifetimeSyntaxesSuggestion!();
    };
}

macro_rules! build_mismatch_suggestion {
    () => {
        deps!();
        fn build_mismatch_suggestion (lifetime_name : & str , infos : & [& Info < '_ >] ,) -> lints :: MismatchedLifetimeSyntaxesSuggestion { let lifetime_name = lifetime_name . to_owned () ; let suggestions = infos . iter () . map (| info | info . suggestion (& lifetime_name)) . collect () ; lints :: MismatchedLifetimeSyntaxesSuggestion :: Explicit { lifetime_name , suggestions , optional_alternative : false , } }
    };
}

build_mismatch_suggestion!();