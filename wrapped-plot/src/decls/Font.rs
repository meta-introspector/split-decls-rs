macro_rules! Font {
    () => {
        # [doc = " A font name"] pub struct Font (Cow < 'static , str >) ;
    };
}

Font!();