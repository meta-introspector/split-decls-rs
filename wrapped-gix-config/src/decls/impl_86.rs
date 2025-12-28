macro_rules! deps {
    () => {
        SectionMut!();
        Header!();
        SectionId!();
        Section!();
        Body!();
        Metadata!();
        Event!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        # [doc = " Access"] impl < 'a > Section < 'a > { # [doc = " Return our header."] pub fn header (& self) -> & section :: Header < 'a > { & self . header } # [doc = " Return the unique `id` of the section, for use with the `*_by_id()` family of methods"] # [doc = " in [`gix_config::File`][crate::File]."] pub fn id (& self) -> SectionId { self . id } # [doc = " Return our body, containing all value names and values."] pub fn body (& self) -> & Body < 'a > { & self . body } # [doc = " Serialize this type into a `BString` for convenience."] # [doc = ""] # [doc = " Note that `to_string()` can also be used, but might not be lossless."] # [must_use] pub fn to_bstring (& self) -> BString { let mut buf = Vec :: new () ; self . write_to (& mut buf) . expect ("io error impossible") ; buf . into () } # [doc = " Stream ourselves to the given `out`, in order to reproduce this section mostly losslessly"] # [doc = " as it was parsed."] pub fn write_to (& self , mut out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { self . header . write_to (& mut * out) ? ; if self . body . 0 . is_empty () { return Ok (()) ; } let nl = self . body . as_ref () . iter () . find_map (extract_newline) . unwrap_or_else (| | platform_newline ()) ; if ! self . body . as_ref () . iter () . take_while (| e | ! matches ! (e , Event :: SectionValueName (_))) . any (| e | e . to_bstr_lossy () . contains_str (nl)) { out . write_all (nl) ? ; } let mut saw_newline_after_value = true ; let mut in_key_value_pair = false ; for (idx , event) in self . body . as_ref () . iter () . enumerate () { match event { Event :: SectionValueName (_) => { if ! saw_newline_after_value { out . write_all (nl) ? ; } saw_newline_after_value = false ; in_key_value_pair = true ; } Event :: Newline (_) if ! in_key_value_pair => { saw_newline_after_value = true ; } Event :: Value (_) | Event :: ValueDone (_) => { in_key_value_pair = false ; } _ => { } } event . write_to (& mut out) ? ; if let Event :: ValueNotDone (_) = event { if self . body . 0 . get (idx + 1) . filter (| e | matches ! (e , Event :: Newline (_))) . is_none () { out . write_all (nl) ? ; } } } Ok (()) } # [doc = " Return additional information about this sections origin."] pub fn meta (& self) -> & Metadata { & self . meta } # [doc = " Returns a mutable version of this section for adjustment of values."] pub fn to_mut (& mut self , newline : SmallVec < u8 , 2 >) -> SectionMut < '_ , 'a > { SectionMut :: new (self , newline) } }
    };
}

impl_86!();