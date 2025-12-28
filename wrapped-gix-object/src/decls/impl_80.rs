macro_rules! deps {
    () => {
        ObjectRef!();
        Blob!();
        Tree!();
        Commit!();
        TagRef!();
        TreeRef!();
        BlobRef!();
        Kind!();
        Tag!();
        CommitRef!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        # [doc = " Convenient access to contained objects."] impl < 'a > ObjectRef < 'a > { # [doc = " Interpret this object as blob."] pub fn as_blob (& self) -> Option < & BlobRef < 'a > > { match self { ObjectRef :: Blob (v) => Some (v) , _ => None , } } # [doc = " Interpret this object as blob, chainable."] pub fn into_blob (self) -> Option < BlobRef < 'a > > { match self { ObjectRef :: Blob (v) => Some (v) , _ => None , } } # [doc = " Interpret this object as commit."] pub fn as_commit (& self) -> Option < & CommitRef < 'a > > { match self { ObjectRef :: Commit (v) => Some (v) , _ => None , } } # [doc = " Interpret this object as commit, chainable."] pub fn into_commit (self) -> Option < CommitRef < 'a > > { match self { ObjectRef :: Commit (v) => Some (v) , _ => None , } } # [doc = " Interpret this object as tree."] pub fn as_tree (& self) -> Option < & TreeRef < 'a > > { match self { ObjectRef :: Tree (v) => Some (v) , _ => None , } } # [doc = " Interpret this object as tree, chainable"] pub fn into_tree (self) -> Option < TreeRef < 'a > > { match self { ObjectRef :: Tree (v) => Some (v) , _ => None , } } # [doc = " Interpret this object as tag."] pub fn as_tag (& self) -> Option < & TagRef < 'a > > { match self { ObjectRef :: Tag (v) => Some (v) , _ => None , } } # [doc = " Interpret this object as tag, chainable."] pub fn into_tag (self) -> Option < TagRef < 'a > > { match self { ObjectRef :: Tag (v) => Some (v) , _ => None , } } # [doc = " Return the kind of object."] pub fn kind (& self) -> Kind { match self { ObjectRef :: Tree (_) => Kind :: Tree , ObjectRef :: Blob (_) => Kind :: Blob , ObjectRef :: Commit (_) => Kind :: Commit , ObjectRef :: Tag (_) => Kind :: Tag , } } }
    };
}

impl_80!();