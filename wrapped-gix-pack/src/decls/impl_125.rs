macro_rules! deps {
    () => {
        Tree!();
        Kind!();
        Offset!();
        Header!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl Header { # [doc = " Subtract `distance` from `pack_offset` safely without the chance for overflow or no-ops if `distance` is 0."] pub fn verified_base_pack_offset (pack_offset : data :: Offset , distance : u64) -> Option < data :: Offset > { if distance == 0 { return None ; } pack_offset . checked_sub (distance) } # [doc = " Convert the header's object kind into [`gix_object::Kind`] if possible"] pub fn as_kind (& self) -> Option < gix_object :: Kind > { use gix_object :: Kind :: * ; Some (match self { Header :: Tree => Tree , Header :: Blob => Blob , Header :: Commit => Commit , Header :: Tag => Tag , Header :: RefDelta { .. } | Header :: OfsDelta { .. } => return None , }) } # [doc = " Convert this header's object kind into the packs internal representation"] pub fn as_type_id (& self) -> u8 { use Header :: * ; match self { Blob => BLOB , Tree => TREE , Commit => COMMIT , Tag => TAG , OfsDelta { .. } => OFS_DELTA , RefDelta { .. } => REF_DELTA , } } # [doc = " Return's true if this is a delta object, i.e. not a full object."] pub fn is_delta (& self) -> bool { matches ! (self , Header :: OfsDelta { .. } | Header :: RefDelta { .. }) } # [doc = " Return's true if this is a base object, i.e. not a delta object."] pub fn is_base (& self) -> bool { ! self . is_delta () } }
    };
}

impl_125!();