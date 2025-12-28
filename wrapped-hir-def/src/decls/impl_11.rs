macro_rules! deps {
    () => {
        DefDatabase!();
        Attrs!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Attrs { pub fn new (db : & dyn DefDatabase , owner : & dyn ast :: HasAttrs , span_map : SpanMapRef < '_ > , cfg_options : & CfgOptions ,) -> Self { Attrs (RawAttrs :: new_expanded (db , owner , span_map , cfg_options)) } pub fn get (& self , id : AttrId) -> Option < & Attr > { (* * self) . iter () . find (| attr | attr . id == id) } pub (crate) fn expand_cfg_attr (db : & dyn DefDatabase , krate : Crate , raw_attrs : RawAttrs ,) -> Attrs { Attrs (raw_attrs . expand_cfg_attr (db , krate)) } pub (crate) fn is_cfg_enabled_for (db : & dyn DefDatabase , owner : & dyn ast :: HasAttrs , span_map : SpanMapRef < '_ > , cfg_options : & CfgOptions ,) -> Result < () , CfgExpr > { RawAttrs :: attrs_iter_expanded :: < false > (db , owner , span_map , cfg_options) . filter_map (| attr | attr . cfg ()) . find_map (| cfg | match cfg_options . check (& cfg) . is_none_or (identity) { true => None , false => Some (cfg) , }) . map_or (Ok (()) , Err) } }
    };
}

impl_11!();