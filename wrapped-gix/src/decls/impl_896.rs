macro_rules! deps {
    () => {
        Repository!();
        Reference!();
        Id!();
        Merge!();
        Note!();
        Spec!();
    };
}

macro_rules! impl_896 {
    () => {
        deps!();
        # [doc = " Access"] impl < 'repo > Spec < 'repo > { # [doc = " Detach the `Repository` from this instance, leaving only plain data that can be moved freely and serialized."] pub fn detach (self) -> gix_revision :: Spec { self . inner } # [doc = " Some revision specifications leave information about references which are returned as `(from-ref, to-ref)` here, e.g."] # [doc = " `HEAD@{-1}..main` might be `(Some(refs/heads/previous-branch), Some(refs/heads/main))`,"] # [doc = " or `@` returns `(Some(refs/heads/main), None)`."] pub fn into_references (self) -> (Option < Reference < 'repo > > , Option < Reference < 'repo > >) { let repo = self . repo ; (self . first_ref . map (| r | r . attach (repo)) , self . second_ref . map (| r | r . attach (repo)) ,) } # [doc = " Return the path encountered in specs like `@:<path>` or `:<path>`, along with the kind of object it represents."] # [doc = ""] # [doc = " Note that there can only be one as paths always terminates further revspec parsing."] pub fn path_and_mode (& self) -> Option < (& BStr , gix_object :: tree :: EntryMode) > { self . path . as_ref () . map (| (p , mode) | (p . as_ref () , * mode)) } # [doc = " Return the name of the first reference we encountered while resolving the rev-spec, or `None` if a short hash"] # [doc = " was used. For example, `@` might yield `Some(HEAD)`, but `abcd` yields `None`."] pub fn first_reference (& self) -> Option < & gix_ref :: Reference > { self . first_ref . as_ref () } # [doc = " Return the name of the second reference we encountered while resolving the rev-spec, or `None` if a short hash"] # [doc = " was used or there was no second reference. For example, `..@` might yield `Some(HEAD)`, but `..abcd` or `@`"] # [doc = " yields `None`."] pub fn second_reference (& self) -> Option < & gix_ref :: Reference > { self . second_ref . as_ref () } # [doc = " Return the single included object represented by this instance, or `None` if it is a range of any kind."] pub fn single (& self) -> Option < Id < 'repo > > { match self . inner { gix_revision :: Spec :: Include (id) | gix_revision :: Spec :: ExcludeParents (id) => { Id :: from_id (id , self . repo) . into () } gix_revision :: Spec :: Exclude (_) | gix_revision :: Spec :: Range { .. } | gix_revision :: Spec :: Merge { .. } | gix_revision :: Spec :: IncludeOnlyParents { .. } => None , } } }
    };
}

impl_896!()