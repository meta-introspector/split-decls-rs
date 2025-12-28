macro_rules! deps {
    () => {
        Reset!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl Reset { # [doc = " Render the ANSI code"] # [doc = ""] # [doc = " `Reset` also implements `Display` directly, so calling this method is optional."] # [inline] pub fn render (self) -> impl core :: fmt :: Display + Copy { self } }
    };
}

impl_44!()