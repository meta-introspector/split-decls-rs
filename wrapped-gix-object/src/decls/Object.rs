macro_rules! deps {
    () => {
        Commit!();
        Tree!();
        Tag!();
        Blob!();
    };
}

macro_rules! Object {
    () => {
        deps!();
        # [doc = " Mutable objects with each field being separately allocated and changeable."] # [doc = ""] # [doc = " Mutable objects are Commits, Trees, Blobs and Tags that can be changed and serialized."] # [doc = ""] # [doc = " They either created using object [construction](Object) or by [deserializing existing objects](ObjectRef::from_bytes())"] # [doc = " and converting these [into mutable copies](ObjectRef::into_owned()) for adjustments."] # [doc = ""] # [doc = " An `Object` is representing [`Trees`](Tree), [`Blobs`](Blob), [`Commits`](Commit), or [`Tags`](Tag)."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: large_enum_variant , missing_docs)] pub enum Object { Tree (Tree) , Blob (Blob) , Commit (Commit) , Tag (Tag) , }
    };
}

Object!();