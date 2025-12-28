macro_rules! deps {
    () => {
        ChangeSet!();
        ColorTag!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a > ColorTag < 'a > { # [doc = " Creates a new close tag; only used in order to auto-close unclosed tags at the end of the"] # [doc = " format string."] pub fn new_close () -> Self { ColorTag { source : None , span : None , is_close : true , change_set : ChangeSet :: default () , } } # [doc = " Sets the span of the tag."] pub fn set_span (& mut self , span : Span) { self . span = Some (span) ; } }
    };
}

impl_48!()