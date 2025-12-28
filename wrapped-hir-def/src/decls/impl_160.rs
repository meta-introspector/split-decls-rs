macro_rules! deps {
    () => {
        TreeId!();
        DefDatabase!();
        ItemTree!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl TreeId { pub (crate) fn new (file : HirFileId , block : Option < BlockId >) -> Self { Self { file , block } } pub (crate) fn item_tree < 'db > (& self , db : & 'db dyn DefDatabase) -> & 'db ItemTree { match self . block { Some (block) => block_item_tree_query (db , block) , None => file_item_tree_query (db , self . file) , } } # [inline] pub fn file_id (self) -> HirFileId { self . file } pub (crate) fn is_block (self) -> bool { self . block . is_some () } }
    };
}

impl_160!();