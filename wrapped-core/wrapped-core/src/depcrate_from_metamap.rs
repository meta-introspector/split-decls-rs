// Generated macro for map (macro)
macro_rules! Depcrate_from_metamap {
() => {
// Module: crate::from_meta
// Provides: {"map"}
// Dependencies: {}
macro_rules ! map { (hash_map , $ key : ty , $ nested : ident) => { impl < V : FromMeta , S : BuildHasher + Default > FromMeta for HashMap <$ key , V , S > { map ! (HashMap :: with_capacity_and_hasher ($ nested . len () , Default :: default ()) , $ key , $ nested) ; } } ; (btree_map , $ key : ty , $ nested : ident) => { impl < V : FromMeta > FromMeta for BTreeMap <$ key , V > { map ! (BTreeMap :: new () , $ key , $ nested) ; } } ; ($ new : expr , $ key : ty , $ nested : ident) => { fn from_list ($ nested : & [NestedMeta]) -> Result < Self > { let pairs = $ nested . iter () . map (| item | -> Result < (& syn :: Path , Result < V >) > { match * item { NestedMeta :: Meta (ref inner) => { let path = inner . path () ; Ok ((path , FromMeta :: from_meta (inner) . map_err (| e | e . at_path (& path)) ,)) } NestedMeta :: Lit (_) => Err (Error :: unsupported_format ("expression")) , } }) ; let mut errors = Error :: accumulator () ; let mut seen_keys = HashSet :: with_capacity ($ nested . len ()) ; let mut map = $ new ; for item in pairs { if let Some ((path , value)) = errors . handle (item) { let key : $ key = match KeyFromPath :: from_path (path) { Ok (k) => k , Err (e) => { errors . push (e) ; errors . handle (value) ; continue ; } } ; let already_seen = seen_keys . contains (& key) ; if already_seen { errors . push (Error :: duplicate_field (& key . to_display ()) . with_span (path)) ; } match value { Ok (_) if already_seen => { } Ok (val) => { map . insert (key . clone () , val) ; } Err (e) => { errors . push (e) ; } } seen_keys . insert (key) ; } } errors . finish_with (map) } } ; }
};
}
