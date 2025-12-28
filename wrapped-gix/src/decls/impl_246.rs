macro_rules! deps {
    () => {
        Object!();
        Commit!();
        Error!();
        Kind!();
        Id!();
        Tag!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        # [doc = " Conversions to detached, lower-level object types."] impl < 'repo > Object < 'repo > { # [doc = " Obtain a fully parsed commit whose fields reference our data buffer,"] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " - this object is not a commit"] # [doc = " - the commit could not be decoded"] pub fn to_commit_ref (& self) -> gix_object :: CommitRef < '_ > { self . try_to_commit_ref () . expect ("BUG: need a commit") } # [doc = " Obtain a fully parsed commit whose fields reference our data buffer."] pub fn try_to_commit_ref (& self) -> Result < gix_object :: CommitRef < '_ > , conversion :: Error > { gix_object :: Data :: new (self . kind , & self . data) . decode () ? . into_commit () . ok_or (conversion :: Error :: UnexpectedType { expected : gix_object :: Kind :: Commit , actual : self . kind , }) } # [doc = " Obtain an iterator over commit tokens like in [`to_commit_iter()`][Object::try_to_commit_ref_iter()]."] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " - this object is not a commit"] pub fn to_commit_ref_iter (& self) -> gix_object :: CommitRefIter < '_ > { gix_object :: Data :: new (self . kind , & self . data) . try_into_commit_iter () . expect ("BUG: This object must be a commit") } # [doc = " Obtain a commit token iterator from the data in this instance, if it is a commit."] pub fn try_to_commit_ref_iter (& self) -> Option < gix_object :: CommitRefIter < '_ > > { gix_object :: Data :: new (self . kind , & self . data) . try_into_commit_iter () } # [doc = " Obtain a tag token iterator from the data in this instance."] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " - this object is not a tag"] pub fn to_tag_ref_iter (& self) -> gix_object :: TagRefIter < '_ > { gix_object :: Data :: new (self . kind , & self . data) . try_into_tag_iter () . expect ("BUG: this object must be a tag") } # [doc = " Obtain a tag token iterator from the data in this instance."] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " - this object is not a tag"] pub fn try_to_tag_ref_iter (& self) -> Option < gix_object :: TagRefIter < '_ > > { gix_object :: Data :: new (self . kind , & self . data) . try_into_tag_iter () } # [doc = " Obtain a tag object from the data in this instance."] # [doc = ""] # [doc = " # Panic"] # [doc = ""] # [doc = " - this object is not a tag"] # [doc = " - the tag could not be decoded"] pub fn to_tag_ref (& self) -> gix_object :: TagRef < '_ > { self . try_to_tag_ref () . expect ("BUG: need tag") } # [doc = " Obtain a fully parsed tag object whose fields reference our data buffer."] pub fn try_to_tag_ref (& self) -> Result < gix_object :: TagRef < '_ > , conversion :: Error > { gix_object :: Data :: new (self . kind , & self . data) . decode () ? . into_tag () . ok_or (conversion :: Error :: UnexpectedType { expected : gix_object :: Kind :: Tag , actual : self . kind , }) } # [doc = " Return the attached id of this object."] pub fn id (& self) -> Id < 'repo > { Id :: from_id (self . id , self . repo) } }
    };
}

impl_246!();