macro_rules! deps {
    () => {
        InternalRef!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl InternalRef { fn unpack_direct (self) -> Option < (BString , gix_hash :: ObjectId) > { match self { InternalRef :: Direct { path , object } => Some ((path , object)) , _ => None , } } fn lookup_symbol_has_path (& self , predicate_path : & BStr) -> bool { matches ! (self , InternalRef :: SymbolicForLookup { path , .. } if path == predicate_path) } }
    };
}

impl_87!()