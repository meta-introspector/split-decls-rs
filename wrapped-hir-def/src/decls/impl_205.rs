macro_rules! deps {
    () => {
        DefDatabase!();
        AdtId!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl LangItem { pub fn resolve_function (self , db : & dyn DefDatabase , start_crate : Crate) -> Option < FunctionId > { lang_item (db , start_crate , self) . and_then (| t | t . as_function ()) } pub fn resolve_trait (self , db : & dyn DefDatabase , start_crate : Crate) -> Option < TraitId > { lang_item (db , start_crate , self) . and_then (| t | t . as_trait ()) } pub fn resolve_adt (self , db : & dyn DefDatabase , start_crate : Crate) -> Option < AdtId > { lang_item (db , start_crate , self) . and_then (| t | t . as_adt ()) } pub fn resolve_enum (self , db : & dyn DefDatabase , start_crate : Crate) -> Option < EnumId > { lang_item (db , start_crate , self) . and_then (| t | t . as_enum ()) } pub fn resolve_type_alias (self , db : & dyn DefDatabase , start_crate : Crate ,) -> Option < TypeAliasId > { lang_item (db , start_crate , self) . and_then (| t | t . as_type_alias ()) } # [doc = " Opposite of [`LangItem::name`]"] pub fn from_name (name : & hir_expand :: name :: Name) -> Option < Self > { Self :: from_symbol (name . symbol ()) } pub fn path (& self , db : & dyn DefDatabase , start_crate : Crate) -> Option < Path > { let t = lang_item (db , start_crate , * self) ? ; Some (Path :: LangItem (t , None)) } pub fn ty_rel_path (& self , db : & dyn DefDatabase , start_crate : Crate , seg : Name) -> Option < Path > { let t = lang_item (db , start_crate , * self) ? ; Some (Path :: LangItem (t , Some (seg))) } }
    };
}

impl_205!()