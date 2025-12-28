macro_rules! deps {
    () => {
        PrettyRef!();
        GitReference!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl GitReference { pub fn from_query (query_pairs : impl Iterator < Item = (impl AsRef < str > , impl AsRef < str >) > ,) -> Self { let mut reference = GitReference :: DefaultBranch ; for (k , v) in query_pairs { let v = v . as_ref () ; match k . as_ref () { "branch" | "ref" => reference = GitReference :: Branch (v . to_owned ()) , "rev" => reference = GitReference :: Rev (v . to_owned ()) , "tag" => reference = GitReference :: Tag (v . to_owned ()) , _ => { } } } reference } # [doc = " Returns a `Display`able view of this git reference, or None if using"] # [doc = " the head of the default branch"] pub fn pretty_ref (& self , url_encoded : bool) -> Option < PrettyRef < '_ > > { match self { GitReference :: DefaultBranch => None , _ => Some (PrettyRef { inner : self , url_encoded , }) , } } }
    };
}

impl_32!()