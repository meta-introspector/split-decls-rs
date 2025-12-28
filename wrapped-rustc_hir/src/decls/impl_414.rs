macro_rules! deps {
    () => {
        Item!();
        LanguageItems!();
    };
}

macro_rules! impl_414 {
    () => {
        deps!();
        impl LanguageItems { # [doc = " Construct an empty collection of lang items and no missing ones."] pub fn new () -> Self { Self { items : [None ; std :: mem :: variant_count :: < LangItem > ()] , reverse_items : FxIndexMap :: default () , missing : Vec :: new () , } } pub fn get (& self , item : LangItem) -> Option < DefId > { self . items [item as usize] } pub fn set (& mut self , item : LangItem , def_id : DefId) { self . items [item as usize] = Some (def_id) ; let preexisting = self . reverse_items . insert (def_id , item) ; if let Some (preexisting) = preexisting { panic ! ("For the bijection of LangItem <=> DefId to work,\
                one item DefId may only be assigned one LangItem. \
                Separate the LangItem definitions for {item:?} and {preexisting:?}.") ; } } pub fn from_def_id (& self , def_id : DefId) -> Option < LangItem > { self . reverse_items . get (& def_id) . copied () } pub fn iter (& self) -> impl Iterator < Item = (LangItem , DefId) > { self . items . iter () . enumerate () . filter_map (| (i , id) | id . map (| id | (LangItem :: from_u32 (i as u32) . unwrap () , id))) } }
    };
}

impl_414!()