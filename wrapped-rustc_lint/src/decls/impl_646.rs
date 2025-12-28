macro_rules! deps {
    () => {
        MismatchedLifetimeSyntaxesSuggestion!();
    };
}

macro_rules! impl_646 {
    () => {
        deps!();
        impl MismatchedLifetimeSyntaxesSuggestion { fn make_optional_alternative (& mut self) { use MismatchedLifetimeSyntaxesSuggestion :: * ; let optional_alternative = match self { Implicit { optional_alternative , .. } | Mixed { optional_alternative , .. } | Explicit { optional_alternative , .. } => optional_alternative , } ; * optional_alternative = true ; } }
    };
}

impl_646!()