macro_rules! deps {
    () => {
        File!();
        Section!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl File < '_ > { # [doc = " Serialize this type into a `BString` for convenience."] # [doc = ""] # [doc = " Note that `to_string()` can also be used, but might not be lossless."] # [must_use] pub fn to_bstring (& self) -> BString { let mut buf = Vec :: new () ; self . write_to (& mut buf) . expect ("io error impossible") ; buf . into () } # [doc = " Stream ourselves to the given `out` in order to reproduce this file mostly losslessly"] # [doc = " as it was parsed, while writing only sections for which `filter` returns true."] pub fn write_to_filter (& self , mut out : & mut dyn std :: io :: Write , mut filter : impl FnMut (& Section < '_ >) -> bool ,) -> std :: io :: Result < () > { let nl = self . detect_newline_style () ; { for event in self . frontmatter_events . as_ref () { event . write_to (& mut out) ? ; } if ! ends_with_newline (self . frontmatter_events . as_ref () , nl , true) && self . sections . values () . any (& mut filter) { out . write_all (nl) ? ; } } let mut prev_section_ended_with_newline = true ; for section_id in & self . section_order { if ! prev_section_ended_with_newline { out . write_all (nl) ? ; } let section = self . sections . get (section_id) . expect ("known section-id") ; if ! filter (section) { continue ; } section . write_to (& mut out) ? ; prev_section_ended_with_newline = ends_with_newline (section . body . 0 . as_ref () , nl , false) ; if let Some (post_matter) = self . frontmatter_post_section . get (section_id) { if ! prev_section_ended_with_newline { out . write_all (nl) ? ; } for event in post_matter { event . write_to (& mut out) ? ; } prev_section_ended_with_newline = ends_with_newline (post_matter , nl , prev_section_ended_with_newline) ; } } if ! prev_section_ended_with_newline { out . write_all (nl) ? ; } Ok (()) } # [doc = " Stream ourselves to the given `out`, in order to reproduce this file mostly losslessly"] # [doc = " as it was parsed."] pub fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { self . write_to_filter (out , | _ | true) } }
    };
}

impl_103!()