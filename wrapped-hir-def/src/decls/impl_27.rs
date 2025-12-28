macro_rules! deps {
    () => {
        AttrSourceMap!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl AttrSourceMap { fn new (owner : InFile < & dyn ast :: HasAttrs >) -> Self { Self { source : collect_attrs (owner . value) . map (| (_ , it) | it) . collect () , file_id : owner . file_id , mod_def_site_file_id : None , } } # [doc = " Append a second source map to this one, this is required for modules, whose outline and inline"] # [doc = " attributes can reside in different files"] fn append_module_inline_attrs (& mut self , other : Self) { assert ! (self . mod_def_site_file_id . is_none () && other . mod_def_site_file_id . is_none ()) ; let len = self . source . len () ; self . source . extend (other . source) ; if other . file_id != self . file_id { self . mod_def_site_file_id = Some ((other . file_id , len)) ; } } # [doc = " Maps the lowered `Attr` back to its original syntax node."] # [doc = ""] # [doc = " `attr` must come from the `owner` used for AttrSourceMap"] # [doc = ""] # [doc = " Note that the returned syntax node might be a `#[cfg_attr]`, or a doc comment, instead of"] # [doc = " the attribute represented by `Attr`."] pub fn source_of (& self , attr : & Attr) -> InFile < & Either < ast :: Attr , ast :: Comment > > { self . source_of_id (attr . id) } pub fn source_of_id (& self , id : AttrId) -> InFile < & Either < ast :: Attr , ast :: Comment > > { let ast_idx = id . ast_index () ; let file_id = match self . mod_def_site_file_id { Some ((file_id , def_site_cut)) if def_site_cut <= ast_idx => file_id , _ => self . file_id , } ; self . source . get (ast_idx) . map (| it | InFile :: new (file_id , it)) . unwrap_or_else (| | panic ! ("cannot find attr at index {id:?}")) } }
    };
}

impl_27!()