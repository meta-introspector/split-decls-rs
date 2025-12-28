macro_rules! deps {
    () => {
        Style!();
        Color!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Color { # [doc = " Create a [`Style`][crate::Style] with this as the foreground"] # [inline] pub fn on (self , background : impl Into < Color >) -> crate :: Style { crate :: Style :: new () . fg_color (Some (self)) . bg_color (Some (background . into ())) } # [doc = " Create a [`Style`][crate::Style] with this as the foreground"] # [inline] pub const fn on_default (self) -> crate :: Style { crate :: Style :: new () . fg_color (Some (self)) } # [doc = " Render the ANSI code for a foreground color"] # [inline] pub fn render_fg (self) -> impl core :: fmt :: Display + Copy { match self { Self :: Ansi (color) => color . as_fg_buffer () , Self :: Ansi256 (color) => color . as_fg_buffer () , Self :: Rgb (color) => color . as_fg_buffer () , } } # [inline] # [cfg (feature = "std")] pub (crate) fn write_fg_to (self , write : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { let buffer = match self { Self :: Ansi (color) => color . as_fg_buffer () , Self :: Ansi256 (color) => color . as_fg_buffer () , Self :: Rgb (color) => color . as_fg_buffer () , } ; buffer . write_to (write) } # [doc = " Render the ANSI code for a background color"] # [inline] pub fn render_bg (self) -> impl core :: fmt :: Display + Copy { match self { Self :: Ansi (color) => color . as_bg_buffer () , Self :: Ansi256 (color) => color . as_bg_buffer () , Self :: Rgb (color) => color . as_bg_buffer () , } } # [inline] # [cfg (feature = "std")] pub (crate) fn write_bg_to (self , write : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { let buffer = match self { Self :: Ansi (color) => color . as_bg_buffer () , Self :: Ansi256 (color) => color . as_bg_buffer () , Self :: Rgb (color) => color . as_bg_buffer () , } ; buffer . write_to (write) } # [inline] pub (crate) fn render_underline (self) -> impl core :: fmt :: Display + Copy { match self { Self :: Ansi (color) => color . as_underline_buffer () , Self :: Ansi256 (color) => color . as_underline_buffer () , Self :: Rgb (color) => color . as_underline_buffer () , } } # [inline] # [cfg (feature = "std")] pub (crate) fn write_underline_to (self , write : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { let buffer = match self { Self :: Ansi (color) => color . as_underline_buffer () , Self :: Ansi256 (color) => color . as_underline_buffer () , Self :: Rgb (color) => color . as_underline_buffer () , } ; buffer . write_to (write) } }
    };
}

impl_3!()