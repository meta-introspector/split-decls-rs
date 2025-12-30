// Generated macro for impl_1152 (impl)
macro_rules! Depcrate_fs_memimpl_1152 {
() => {
// Module: crate::fs::mem
// Provides: {"impl_1152"}
// Dependencies: {}
impl MemDirectory { pub fn new (mode : AccessPermission) -> Self { let microseconds = arch :: kernel :: systemtime :: now_micros () ; let t = timespec :: from_usec (microseconds as i64) ; Self { inner : Arc :: new (RwLock :: new (BTreeMap :: new ())) , attr : FileAttr { st_mode : mode | AccessPermission :: S_IFDIR , st_atim : t , st_mtim : t , st_ctim : t , .. Default :: default () } , } } async fn async_traverse_open (& self , components : & mut Vec < & str > , opt : OpenOption , mode : AccessPermission ,) -> io :: Result < Arc < async_lock :: RwLock < dyn ObjectInterface > > > { if let Some (component) = components . pop () { let node_name = String :: from (component) ; if components . is_empty () { let mut guard = self . inner . write () . await ; if let Some (file) = guard . get (& node_name) { if opt . contains (OpenOption :: O_DIRECTORY) && file . get_kind () != NodeKind :: Directory { return Err (Errno :: Notdir) ; } if file . get_kind () == NodeKind :: File || file . get_kind () == NodeKind :: Directory { return file . get_object () ; } else { return Err (Errno :: Noent) ; } } else if opt . contains (OpenOption :: O_CREAT) { let file = Box :: new (RamFile :: new (mode)) ; guard . insert (node_name , file . clone ()) ; return Ok (Arc :: new (async_lock :: RwLock :: new (RamFileInterface :: new (file . data . clone () ,)))) ; } else { return Err (Errno :: Noent) ; } } if let Some (directory) = self . inner . read () . await . get (& node_name) { return directory . traverse_open (components , opt , mode) ; } } Err (Errno :: Noent) } }
};
}
