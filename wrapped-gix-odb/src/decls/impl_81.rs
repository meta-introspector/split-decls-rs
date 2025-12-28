macro_rules! deps {
    () => {
        IndexAndPacks!();
        Either!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl Either { fn path (& self) -> & Path { match self { Either :: IndexPath (p) => p , Either :: MultiIndexFile (f) => f . path () , } } fn into_index_and_packs (self , mtime : SystemTime) -> IndexAndPacks { match self { Either :: IndexPath (path) => IndexAndPacks :: new_single (path , mtime) , Either :: MultiIndexFile (file) => IndexAndPacks :: new_multi_from_open_file (file , mtime) , } } fn is_multi_index (& self) -> bool { matches ! (self , Either :: MultiIndexFile (_)) } }
    };
}

impl_81!()