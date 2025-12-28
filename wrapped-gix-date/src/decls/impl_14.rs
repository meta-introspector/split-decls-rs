macro_rules! deps {
    () => {
        Format!();
        Error!();
        CustomFormat!();
        Time!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [doc = " Formatting"] impl Time { # [doc = " Format this instance according to the given `format`."] # [doc = ""] # [doc = " Use [`Format::Unix`], [`Format::Raw`] or one of the custom formats"] # [doc = " defined in the [`format`](mod@crate::time::format) submodule."] # [doc = ""] # [doc = " Note that this can fail if the timezone isn't valid and the format requires a conversion to [`jiff::Zoned`]."] pub fn format (& self , format : impl Into < Format >) -> Result < String , jiff :: Error > { self . format_inner (format . into ()) } # [doc = " Like [`Self::format()`], but on time conversion error, produce the [UNIX] format instead"] # [doc = " to make it infallible."] pub fn format_or_unix (& self , format : impl Into < Format >) -> String { self . format_inner (format . into ()) . unwrap_or_else (| _ | self . seconds . to_string ()) } fn format_inner (& self , format : Format) -> Result < String , jiff :: Error > { Ok (match format { Format :: Custom (CustomFormat (format)) => self . to_zoned () ? . strftime (format) . to_string () , Format :: Unix => self . seconds . to_string () , Format :: Raw => self . to_string () , }) } }
    };
}

impl_14!();