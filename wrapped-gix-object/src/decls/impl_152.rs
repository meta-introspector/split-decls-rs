macro_rules! deps {
    () => {
        TagRefIter!();
        Data!();
        Commit!();
        Tag!();
        TreeRef!();
        BlobRef!();
        Tree!();
        TagRef!();
        TreeRefIter!();
        Blob!();
        ObjectRef!();
        CommitRef!();
        Error!();
        Kind!();
        CommitRefIter!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < 'a > Data < 'a > { # [doc = " Constructs a new data object from `kind` and `data`."] pub fn new (kind : Kind , data : & 'a [u8]) -> Data < 'a > { Data { kind , data } } # [doc = " Decodes the data in the backing slice into a [`ObjectRef`], allowing to access all of its data"] # [doc = " conveniently. The cost of parsing an object is negligible."] # [doc = ""] # [doc = " **Note** that [mutable, decoded objects][crate::Object] can be created from [`Data`]"] # [doc = " using [`crate::ObjectRef::into_owned()`]."] pub fn decode (& self) -> Result < ObjectRef < 'a > , crate :: decode :: Error > { Ok (match self . kind { Kind :: Tree => ObjectRef :: Tree (TreeRef :: from_bytes (self . data) ?) , Kind :: Blob => ObjectRef :: Blob (BlobRef { data : self . data }) , Kind :: Commit => ObjectRef :: Commit (CommitRef :: from_bytes (self . data) ?) , Kind :: Tag => ObjectRef :: Tag (TagRef :: from_bytes (self . data) ?) , }) } # [doc = " Returns this object as tree iterator to parse entries one at a time to avoid allocations, or"] # [doc = " `None` if this is not a tree object."] pub fn try_into_tree_iter (self) -> Option < TreeRefIter < 'a > > { match self . kind { Kind :: Tree => Some (TreeRefIter :: from_bytes (self . data)) , _ => None , } } # [doc = " Returns this object as commit iterator to parse tokens one at a time to avoid allocations, or"] # [doc = " `None` if this is not a commit object."] pub fn try_into_commit_iter (self) -> Option < CommitRefIter < 'a > > { match self . kind { Kind :: Commit => Some (CommitRefIter :: from_bytes (self . data)) , _ => None , } } # [doc = " Returns this object as tag iterator to parse tokens one at a time to avoid allocations, or"] # [doc = " `None` if this is not a tag object."] pub fn try_into_tag_iter (self) -> Option < TagRefIter < 'a > > { match self . kind { Kind :: Tag => Some (TagRefIter :: from_bytes (self . data)) , _ => None , } } }
    };
}

impl_152!();