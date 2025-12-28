macro_rules! deps {
    () => {
        SearchMode!();
        Pattern!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [doc = " Access"] impl Pattern { # [doc = " Return `true` if this pathspec is negated, which means it will exclude an item from the result set instead of including it."] pub fn is_excluded (& self) -> bool { self . signature . contains (MagicSignature :: EXCLUDE) } # [doc = " Returns `true` is this pattern is supposed to always match, as it's either empty or designated `nil`."] # [doc = " Note that technically the pattern might still be excluded."] pub fn always_matches (& self) -> bool { self . is_nil () || self . path . is_empty () } # [doc = " Translate ourselves to a long display format, that when parsed back will yield the same pattern."] # [doc = ""] # [doc = " Note that the"] pub fn to_bstring (& self) -> BString { if self . is_nil () { ":" . into () } else { let mut buf : BString = ":(" . into () ; if self . signature . contains (MagicSignature :: TOP) { buf . push_str ("top,") ; } if self . signature . contains (MagicSignature :: EXCLUDE) { buf . push_str ("exclude,") ; } if self . signature . contains (MagicSignature :: ICASE) { buf . push_str ("icase,") ; } match self . search_mode { SearchMode :: ShellGlob => { } SearchMode :: Literal => buf . push_str ("literal,") , SearchMode :: PathAwareGlob => buf . push_str ("glob,") , } if self . attributes . is_empty () { if buf . last () == Some (& b',') { buf . pop () ; } } else { buf . push_str ("attr:") ; for attr in & self . attributes { let attr = attr . as_ref () . to_string () . replace (',' , r"\,") ; buf . push_str (& attr) ; buf . push (b' ') ; } buf . pop () ; } buf . push (b')') ; buf . extend_from_slice (& self . path) ; if self . signature . contains (MagicSignature :: MUST_BE_DIR) { buf . push (b'/') ; } buf } } }
    };
}

impl_3!();