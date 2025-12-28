macro_rules! deps {
    () => {
        DefKey!();
        Item!();
        DefPath!();
        DefPathTable!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl DefPathTable { fn new (stable_crate_id : StableCrateId) -> DefPathTable { DefPathTable { stable_crate_id , index_to_key : Default :: default () , def_path_hashes : Default :: default () , def_path_hash_to_index : Default :: default () , } } fn allocate (& mut self , key : DefKey , def_path_hash : DefPathHash) -> DefIndex { debug_assert_eq ! (self . stable_crate_id , def_path_hash . stable_crate_id ()) ; let local_hash = def_path_hash . local_hash () ; let index = self . index_to_key . push (key) ; debug ! ("DefPathTable::insert() - {key:?} <-> {index:?}") ; self . def_path_hashes . push (local_hash) ; debug_assert ! (self . def_path_hashes . len () == self . index_to_key . len ()) ; if let Some (existing) = self . def_path_hash_to_index . insert (& local_hash , & index) { let def_path1 = DefPath :: make (LOCAL_CRATE , existing , | idx | self . def_key (idx)) ; let def_path2 = DefPath :: make (LOCAL_CRATE , index , | idx | self . def_key (idx)) ; panic ! ("found DefPathHash collision between {def_path1:#?} and {def_path2:#?}. \
                    Compilation cannot continue.") ; } index } # [inline (always)] pub fn def_key (& self , index : DefIndex) -> DefKey { self . index_to_key [index] } # [instrument (level = "trace" , skip (self) , ret)] # [inline (always)] pub fn def_path_hash (& self , index : DefIndex) -> DefPathHash { let hash = self . def_path_hashes [index] ; DefPathHash :: new (self . stable_crate_id , hash) } pub fn enumerated_keys_and_path_hashes (& self ,) -> impl Iterator < Item = (DefIndex , & DefKey , DefPathHash) > + ExactSizeIterator { self . index_to_key . iter_enumerated () . map (move | (index , key) | (index , key , self . def_path_hash (index))) } }
    };
}

impl_84!()