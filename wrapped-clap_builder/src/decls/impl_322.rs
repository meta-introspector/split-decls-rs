macro_rules! deps {
    () => {
        UnknownArgumentValueParser!();
        StyledStr!();
        Str!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl UnknownArgumentValueParser { # [doc = " Suggest an alternative argument"] pub fn suggest_arg (arg : impl Into < Str >) -> Self { Self { arg : Some (arg . into ()) , suggestions : Default :: default () , } } # [doc = " Provide a general suggestion"] pub fn suggest (text : impl Into < StyledStr >) -> Self { Self { arg : Default :: default () , suggestions : vec ! [text . into ()] , } } # [doc = " Extend the suggestions"] pub fn and_suggest (mut self , text : impl Into < StyledStr >) -> Self { self . suggestions . push (text . into ()) ; self } }
    };
}

impl_322!()