macro_rules! deps {
    () => {
        Note!();
        Rewrite!();
        Fetch!();
        Push!();
        Url!();
        Replace!();
        Direction!();
    };
}

macro_rules! impl_978 {
    () => {
        deps!();
        # [doc = " Access"] impl Rewrite { fn replacements_for (& self , direction : Direction) -> & [Replace] { match direction { Direction :: Fetch => & self . url_rewrite , Direction :: Push => & self . push_url_rewrite , } } pub fn longest (& self , url : & gix_url :: Url , direction : Direction) -> Option < BString > { if self . replacements_for (direction) . is_empty () { None } else { let mut url = url . to_bstring () ; self . rewrite_url_in_place (& mut url , direction) . then_some (url) } } # [doc = " Rewrite the given `url` of `direction` and return `true` if a replacement happened."] # [doc = ""] # [doc = " Note that the result must still be checked for validity, it might not be a valid URL as we do a syntax-unaware replacement."] pub fn rewrite_url_in_place (& self , url : & mut BString , direction : Direction) -> bool { self . replacements_for (direction) . iter () . fold (None :: < (usize , & BStr) > , | mut acc , replace | { if url . starts_with (replace . find . as_ref ()) { let (bytes_matched , prev_rewrite_with) = acc . get_or_insert ((replace . find . len () , replace . with . as_slice () . into ())) ; if * bytes_matched < replace . find . len () { * bytes_matched = replace . find . len () ; * prev_rewrite_with = replace . with . as_slice () . into () ; } } acc }) . map (| (bytes_matched , replace_with) | { url . replace_range (.. bytes_matched , replace_with) ; }) . is_some () } }
    };
}

impl_978!()