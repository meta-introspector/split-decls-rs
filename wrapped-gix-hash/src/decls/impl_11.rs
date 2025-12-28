macro_rules! deps {
    () => {
        Kind!();
        ObjectId!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [doc = " Access and conversion"] impl ObjectId { # [doc = " Returns the kind of hash used in this instance."] # [inline] pub fn kind (& self) -> Kind { match self { ObjectId :: Sha1 (_) => Kind :: Sha1 , } } # [doc = " Return the raw byte slice representing this hash."] # [inline] pub fn as_slice (& self) -> & [u8] { match self { Self :: Sha1 (b) => b . as_ref () , } } # [doc = " Return the raw mutable byte slice representing this hash."] # [inline] pub fn as_mut_slice (& mut self) -> & mut [u8] { match self { Self :: Sha1 (b) => b . as_mut () , } } # [doc = " The hash of an empty blob."] # [inline] pub const fn empty_blob (hash : Kind) -> ObjectId { match hash { Kind :: Sha1 => { ObjectId :: Sha1 (* b"\xe6\x9d\xe2\x9b\xb2\xd1\xd6\x43\x4b\x8b\x29\xae\x77\x5a\xd8\xc2\xe4\x8c\x53\x91") } } } # [doc = " The hash of an empty tree."] # [inline] pub const fn empty_tree (hash : Kind) -> ObjectId { match hash { Kind :: Sha1 => { ObjectId :: Sha1 (* b"\x4b\x82\x5d\xc6\x42\xcb\x6e\xb9\xa0\x60\xe5\x4b\xf8\xd6\x92\x88\xfb\xee\x49\x04") } } } # [doc = " Returns an instances whose bytes are all zero."] # [inline] # [doc (alias = "zero" , alias = "git2")] pub const fn null (kind : Kind) -> ObjectId { match kind { Kind :: Sha1 => Self :: null_sha1 () , } } # [doc = " Returns `true` if this hash consists of all null bytes."] # [inline] # [doc (alias = "is_zero" , alias = "git2")] pub fn is_null (& self) -> bool { match self { ObjectId :: Sha1 (digest) => & digest [..] == oid :: null_sha1 () . as_bytes () , } } # [doc = " Returns `true` if this hash is equal to an empty blob."] # [inline] pub fn is_empty_blob (& self) -> bool { self == & Self :: empty_blob (self . kind ()) } # [doc = " Returns `true` if this hash is equal to an empty tree."] # [inline] pub fn is_empty_tree (& self) -> bool { self == & Self :: empty_tree (self . kind ()) } }
    };
}

impl_11!();