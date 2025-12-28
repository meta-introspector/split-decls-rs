macro_rules! deps {
    () => {
        Tag!();
        BlobRef!();
        Commit!();
        Blob!();
        Kind!();
        Tree!();
        Object!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        # [doc = " Convenient extraction of typed object."] impl Object { # [doc = " Turns this instance into a [`Blob`], panic otherwise."] pub fn into_blob (self) -> Blob { match self { Object :: Blob (v) => v , _ => panic ! ("BUG: not a blob") , } } # [doc = " Turns this instance into a [`Commit`] panic otherwise."] pub fn into_commit (self) -> Commit { match self { Object :: Commit (v) => v , _ => panic ! ("BUG: not a commit") , } } # [doc = " Turns this instance into a [`Tree`] panic otherwise."] pub fn into_tree (self) -> Tree { match self { Object :: Tree (v) => v , _ => panic ! ("BUG: not a tree") , } } # [doc = " Turns this instance into a [`Tag`] panic otherwise."] pub fn into_tag (self) -> Tag { match self { Object :: Tag (v) => v , _ => panic ! ("BUG: not a tag") , } } # [doc = " Turns this instance into a [`Blob`] if it is one."] # [allow (clippy :: result_large_err)] pub fn try_into_blob (self) -> Result < Blob , Self > { match self { Object :: Blob (v) => Ok (v) , _ => Err (self) , } } # [doc = " Turns this instance into a [`BlobRef`] if it is a blob."] pub fn try_into_blob_ref (& self) -> Option < BlobRef < '_ > > { match self { Object :: Blob (v) => Some (v . to_ref ()) , _ => None , } } # [doc = " Turns this instance into a [`Commit`] if it is one."] # [allow (clippy :: result_large_err)] pub fn try_into_commit (self) -> Result < Commit , Self > { match self { Object :: Commit (v) => Ok (v) , _ => Err (self) , } } # [doc = " Turns this instance into a [`Tree`] if it is one."] # [allow (clippy :: result_large_err)] pub fn try_into_tree (self) -> Result < Tree , Self > { match self { Object :: Tree (v) => Ok (v) , _ => Err (self) , } } # [doc = " Turns this instance into a [`Tag`] if it is one."] # [allow (clippy :: result_large_err)] pub fn try_into_tag (self) -> Result < Tag , Self > { match self { Object :: Tag (v) => Ok (v) , _ => Err (self) , } } # [doc = " Returns a [`Blob`] if it is one."] pub fn as_blob (& self) -> Option < & Blob > { match self { Object :: Blob (v) => Some (v) , _ => None , } } # [doc = " Returns a [`Commit`] if it is one."] pub fn as_commit (& self) -> Option < & Commit > { match self { Object :: Commit (v) => Some (v) , _ => None , } } # [doc = " Returns a [`Tree`] if it is one."] pub fn as_tree (& self) -> Option < & Tree > { match self { Object :: Tree (v) => Some (v) , _ => None , } } # [doc = " Returns a [`Tag`] if it is one."] pub fn as_tag (& self) -> Option < & Tag > { match self { Object :: Tag (v) => Some (v) , _ => None , } } # [doc = " Returns the kind of object stored in this instance."] pub fn kind (& self) -> crate :: Kind { match self { Object :: Tree (_) => crate :: Kind :: Tree , Object :: Blob (_) => crate :: Kind :: Blob , Object :: Commit (_) => crate :: Kind :: Commit , Object :: Tag (_) => crate :: Kind :: Tag , } } }
    };
}

impl_77!();