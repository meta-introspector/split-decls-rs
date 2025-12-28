macro_rules! deps {
    () => {
        SuggestionStyle!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl SuggestionStyle { fn hide_inline (& self) -> bool { ! matches ! (* self , SuggestionStyle :: ShowCode) } }
    };
}

impl_23!()