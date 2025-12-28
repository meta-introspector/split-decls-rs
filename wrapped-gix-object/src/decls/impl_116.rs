macro_rules! deps {
    () => {
        Kind!();
        EntryRef!();
        Find!();
        Error!();
        TreeRef!();
        Tree!();
        EntryKind!();
        Blob!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < 'a > TreeRef < 'a > { # [doc = " Deserialize a Tree from `data`."] pub fn from_bytes (mut data : & 'a [u8]) -> Result < TreeRef < 'a > , crate :: decode :: Error > { let input = & mut data ; match decode :: tree . parse_next (input) { Ok (tag) => Ok (tag) , Err (err) => Err (crate :: decode :: Error :: with_err (err , input)) , } } # [doc = " Find an entry named `name` knowing if the entry is a directory or not, using a binary search."] # [doc = ""] # [doc = " Note that it's impossible to binary search by name alone as the sort order is special."] pub fn bisect_entry (& self , name : & BStr , is_dir : bool) -> Option < EntryRef < 'a > > { static NULL_HASH : gix_hash :: ObjectId = gix_hash :: Kind :: shortest () . null () ; let search = EntryRef { mode : if is_dir { tree :: EntryKind :: Tree } else { tree :: EntryKind :: Blob } . into () , filename : name , oid : & NULL_HASH , } ; self . entries . binary_search_by (| e | e . cmp (& search)) . ok () . map (| idx | self . entries [idx]) } # [doc = " Create an instance of the empty tree."] # [doc = ""] # [doc = " It's particularly useful as static part of a program."] pub const fn empty () -> TreeRef < 'static > { TreeRef { entries : Vec :: new () } } }
    };
}

impl_116!()