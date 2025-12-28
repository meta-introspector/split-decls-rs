macro_rules! deps {
    () => {
        NameRef!();
        MatchKind!();
        AttributeId!();
        Outcome!();
        Match!();
        AssignmentRef!();
        StateRef!();
        MatchLocation!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [doc = " Access"] impl Outcome { # [doc = " Return an iterator over all filled attributes we were initialized with."] # [doc = ""] # [doc = " ### Note"] # [doc = ""] # [doc = " If [`initialize_with_selection`][Self::initialize_with_selection()] was used,"] # [doc = " use [`iter_selected()`][Self::iter_selected()] instead."] # [doc = ""] # [doc = " ### Deviation"] # [doc = ""] # [doc = " It's possible that the order in which the attribute are returned (if not limited to a set of attributes) isn't exactly"] # [doc = " the same as what `git` provides."] # [doc = " Ours is in order of declaration, whereas `git` seems to list macros first somehow. Since the values are the same, this"] # [doc = " shouldn't be an issue."] pub fn iter (& self) -> impl Iterator < Item = crate :: search :: Match < '_ > > { self . matches_by_id . iter () . filter_map (| item | item . r#match . as_ref () . map (| m | m . to_outer (self))) } # [doc = " Iterate over all matches of the attribute selection in their original order."] # [doc = ""] # [doc = " This only yields values if this instance was initialized with [`Outcome::initialize_with_selection()`]."] pub fn iter_selected (& self) -> impl Iterator < Item = crate :: search :: Match < '_ > > { static DUMMY : Pattern = Pattern { text : BString :: new (Vec :: new ()) , mode : gix_glob :: pattern :: Mode :: empty () , first_wildcard_pos : None , } ; self . selected . iter () . map (| (name , id) | { id . and_then (| id | self . matches_by_id [id . 0] . r#match . as_ref () . map (| m | m . to_outer (self))) . unwrap_or_else (| | crate :: search :: Match { pattern : & DUMMY , assignment : AssignmentRef { name : NameRef :: try_from (name . as_bytes () . as_bstr ()) . unwrap_or_else (| _ | NameRef ("invalid" . into ())) , state : StateRef :: Unspecified , } , kind : MatchKind :: Attribute { macro_id : None } , location : crate :: search :: MatchLocation { source : None , sequence_number : 0 , } , }) }) } # [doc = " Obtain a match by the order of its attribute, if the order exists in our initialized attribute list and there was a match."] pub fn match_by_id (& self , id : AttributeId) -> Option < crate :: search :: Match < '_ > > { self . matches_by_id . get (id . 0) . and_then (| m | m . r#match . as_ref () . map (| m | m . to_outer (self))) } # [doc = " Return `true` if there is nothing more to be done as all attributes were filled."] pub fn is_done (& self) -> bool { self . remaining () == 0 } }
    };
}

impl_35!()