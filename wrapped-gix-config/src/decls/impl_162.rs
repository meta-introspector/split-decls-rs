macro_rules! deps {
    () => {
        Event!();
        Header!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl Header < '_ > { # [doc = "Return true if this is a header like `[legacy.subsection]`, or false otherwise."] pub fn is_legacy (& self) -> bool { self . separator . as_deref () . is_some_and (| n | n == ".") } # [doc = " Return the subsection name, if present, i.e. \"origin\" in `[remote \"origin\"]`."] # [doc = ""] # [doc = " It is parsed without quotes, and with escapes folded"] # [doc = " into their resulting characters."] # [doc = " Thus during serialization, escapes and quotes must be re-added."] # [doc = " This makes it possible to use [`Event`] data for lookups directly."] pub fn subsection_name (& self) -> Option < & BStr > { self . subsection_name . as_deref () } # [doc = " Return the name of the header, like \"remote\" in `[remote \"origin\"]`."] pub fn name (& self) -> & BStr { & self . name } # [doc = " Serialize this type into a `BString` for convenience."] # [doc = ""] # [doc = " Note that `to_string()` can also be used, but might not be lossless."] # [must_use] pub fn to_bstring (& self) -> BString { let mut buf = Vec :: new () ; self . write_to (& mut buf) . expect ("io error impossible") ; buf . into () } # [doc = " Stream ourselves to the given `out`, in order to reproduce this header mostly losslessly"] # [doc = " as it was parsed."] pub fn write_to (& self , mut out : impl std :: io :: Write) -> std :: io :: Result < () > { out . write_all (b"[") ? ; out . write_all (& self . name) ? ; if let (Some (sep) , Some (subsection)) = (& self . separator , & self . subsection_name) { let sep = sep . as_ref () ; out . write_all (sep) ? ; if sep == "." { out . write_all (subsection . as_ref ()) ? ; } else { out . write_all (b"\"") ? ; out . write_all (escape_subsection (subsection . as_ref ()) . as_ref ()) ? ; out . write_all (b"\"") ? ; } } out . write_all (b"]") } # [doc = " Turn this instance into a fully owned one with `'static` lifetime."] # [must_use] pub fn to_owned (& self) -> Header < 'static > { Header { name : self . name . to_owned () , separator : self . separator . clone () . map (| v | Cow :: Owned (v . into_owned ())) , subsection_name : self . subsection_name . clone () . map (| v | Cow :: Owned (v . into_owned ())) , } } }
    };
}

impl_162!()