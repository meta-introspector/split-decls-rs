macro_rules! deps {
    () => {
        Color!();
        Error!();
        Name!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl TryFrom < & BStr > for Color { type Error = Error ; fn try_from (s : & BStr) -> Result < Self , Self :: Error > { let s = std :: str :: from_utf8 (s) . map_err (| err | color_err (s) . with_err (err)) ? ; enum ColorItem { Value (Name) , Attr (Attribute) , } let items = s . split_whitespace () . filter_map (| s | { if s . is_empty () { return None ; } Some (Name :: from_str (s) . map (ColorItem :: Value) . or_else (| _ | Attribute :: from_str (s) . map (ColorItem :: Attr)) ,) }) ; let mut foreground = None ; let mut background = None ; let mut attributes = Attribute :: empty () ; for item in items { match item { Ok (item) => match item { ColorItem :: Value (v) => { if foreground . is_none () { foreground = Some (v) ; } else if background . is_none () { background = Some (v) ; } else { return Err (color_err (s)) ; } } ColorItem :: Attr (a) => attributes |= a , } , Err (_) => return Err (color_err (s)) , } } Ok (Color { foreground , background , attributes , }) } }
    };
}

impl_15!()