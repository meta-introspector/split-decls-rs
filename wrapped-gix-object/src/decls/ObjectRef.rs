macro_rules! deps {
    () => {
        Blob!();
        CommitRef!();
        TagRef!();
        TreeRef!();
        BlobRef!();
        Commit!();
        Tree!();
        Tag!();
    };
}

macro_rules! ObjectRef {
    () => {
        deps!();
        # [doc = " Immutable objects are read-only structures referencing most data from [a byte slice](ObjectRef::from_bytes())."] # [doc = ""] # [doc = " Immutable objects are expected to be deserialized from bytes that acts as backing store, and they"] # [doc = " cannot be mutated or serialized. Instead, one will [convert](ObjectRef::into_owned()) them into their [`mutable`](Object) counterparts"] # [doc = " which support mutation and serialization."] # [doc = ""] # [doc = " An `ObjectRef` is representing [`Trees`](TreeRef), [`Blobs`](BlobRef), [`Commits`](CommitRef), or [`Tags`](TagRef)."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (missing_docs)] pub enum ObjectRef < 'a > { # [cfg_attr (feature = "serde" , serde (borrow))] Tree (TreeRef < 'a >) , Blob (BlobRef < 'a >) , Commit (CommitRef < 'a >) , Tag (TagRef < 'a >) , }
    };
}

ObjectRef!()