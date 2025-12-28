macro_rules! deps {
    () => {
        FullNameRef!();
        Category!();
        PartialNameRef!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl FullNameRef { # [doc = " Interpret this fully qualified reference name as partial name."] pub fn as_partial_name (& self) -> & PartialNameRef { PartialNameRef :: new_unchecked (self . 0 . as_bstr ()) } # [doc = " Convert this name into the relative path identifying the reference location."] pub fn to_path (& self) -> & Path { gix_path :: from_byte_slice (& self . 0) } # [doc = " Return ourselves as byte string which is a valid refname"] pub fn as_bstr (& self) -> & BStr { & self . 0 } # [doc = " Strip well-known prefixes from the name and return it."] # [doc = ""] # [doc = " If there is no such prefix, the original name is returned."] pub fn shorten (& self) -> & BStr { self . category_and_short_name () . map_or_else (| | self . 0 . as_bstr () , | (_ , short) | short) } # [doc = " Classify this name, or return `None` if it's unclassified."] pub fn category (& self) -> Option < Category < '_ > > { self . category_and_short_name () . map (| (cat , _) | cat) } # [doc = " Classify this name, or return `None` if it's unclassified. If `Some`,"] # [doc = " the shortened name is returned as well."] pub fn category_and_short_name (& self) -> Option < (Category < '_ > , & BStr) > { let name = self . 0 . as_bstr () ; for category in & [Category :: Tag , Category :: LocalBranch , Category :: RemoteBranch] { if let Some (shortened) = name . strip_prefix (category . prefix () . as_bytes ()) { return Some ((* category , shortened . as_bstr ())) ; } } for category in & [Category :: Note , Category :: Bisect , Category :: WorktreePrivate , Category :: Rewritten ,] { if name . starts_with (category . prefix () . as_ref ()) { return Some ((* category , name . strip_prefix (b"refs/") . expect ("we checked for refs/* above") . as_bstr () ,)) ; } } if is_pseudo_ref (name) { Some ((Category :: PseudoRef , name)) } else if let Some (shortened) = name . strip_prefix (Category :: MainPseudoRef . prefix () . as_bytes ()) { if shortened . starts_with_str ("refs/") { (Category :: MainRef , shortened . as_bstr ()) . into () } else { is_pseudo_ref (shortened . into ()) . then (| | (Category :: MainPseudoRef , shortened . as_bstr ())) } } else if let Some (shortened_with_worktree_name) = name . strip_prefix (Category :: LinkedPseudoRef { name : "" . into () } . prefix () . as_bytes ()) { let (name , shortened) = shortened_with_worktree_name . find_byte (b'/') . map (| pos | { (shortened_with_worktree_name [.. pos] . as_bstr () , shortened_with_worktree_name [pos + 1 ..] . as_bstr () ,) }) ? ; if shortened . starts_with_str ("refs/") { (Category :: LinkedRef { name } , shortened . as_bstr ()) . into () } else { is_pseudo_ref (shortened) . then (| | (Category :: LinkedPseudoRef { name } , shortened . as_bstr ())) } } else { None } } }
    };
}

impl_11!();