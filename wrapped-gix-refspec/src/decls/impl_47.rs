macro_rules! deps {
    () => {
        Item!();
        Match!();
        Mode!();
        Needle!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a > Needle < 'a > { # [inline] fn matches (& self , item : Item < '_ >) -> Match { match self { Needle :: FullName (name) => { if * name == item . full_ref_name { Match :: Normal } else { Match :: None } } Needle :: PartialName (name) => crate :: spec :: expand_partial_name (name , | expanded | { (expanded == item . full_ref_name) . then_some (Match :: Normal) }) . unwrap_or (Match :: None) , Needle :: Glob { name , asterisk_pos } => { match item . full_ref_name . get (.. * asterisk_pos) { Some (full_name_portion) if full_name_portion != name [.. * asterisk_pos] => { return Match :: None ; } None => return Match :: None , _ => { } } let tail = & name [* asterisk_pos + 1 ..] ; if ! item . full_ref_name . ends_with (tail) { return Match :: None ; } let end = item . full_ref_name . len () - tail . len () ; Match :: GlobRange (* asterisk_pos .. end) } Needle :: Pattern (pattern) => { if gix_glob :: wildmatch (pattern , item . full_ref_name , gix_glob :: wildmatch :: Mode :: NO_MATCH_SLASH_LITERAL ,) { Match :: Normal } else { Match :: None } } Needle :: Object (id) => { if * id == item . target { return Match :: Normal ; } match item . object { Some (object) if object == * id => Match :: Normal , _ => Match :: None , } } } } fn to_bstr_replace (self , range : Option < (Range < usize > , Item < '_ >) >) -> Cow < 'a , BStr > { match (self , range) { (Needle :: FullName (name) , None) => Cow :: Borrowed (name) , (Needle :: PartialName (name) , None) => Cow :: Owned ({ let mut base : BString = "refs/" . into () ; if ! (name . starts_with (b"tags/") || name . starts_with (b"remotes/")) { base . push_str ("heads/") ; } base . push_str (name) ; base }) , (Needle :: Glob { name , asterisk_pos } , Some ((range , item))) => { let mut buf = Vec :: with_capacity (name . len () + range . len () - 1) ; buf . push_str (& name [.. asterisk_pos]) ; buf . push_str (& item . full_ref_name [range]) ; buf . push_str (& name [asterisk_pos + 1 ..]) ; Cow :: Owned (buf . into ()) } (Needle :: Object (id) , None) => { let mut name = id . to_string () ; name . insert_str (0 , "refs/heads/") ; Cow :: Owned (name . into ()) } (Needle :: Pattern (name) , None) => Cow :: Borrowed (name) , (Needle :: Glob { .. } , None) => unreachable ! ("BUG: no range provided for glob pattern") , (Needle :: Pattern (_) , Some (_)) => { unreachable ! ("BUG: range provided for pattern, but patterns don't use ranges") } (_ , Some (_)) => { unreachable ! ("BUG: range provided even though needle wasn't a glob. Globs are symmetric.") } } } pub fn to_bstr (self) -> Cow < 'a , BStr > { self . to_bstr_replace (None) } }
    };
}

impl_47!()