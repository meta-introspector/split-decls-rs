// Generated macro for tests (module)
macro_rules! Depcrate_blobtests {
() => {
// Module: crate::blob
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: Repository ; use std :: fs :: File ; use std :: io :: prelude :: * ; use std :: path :: Path ; use tempfile :: TempDir ; # [test] fn buffer () { let td = TempDir :: new () . unwrap () ; let repo = Repository :: init (td . path ()) . unwrap () ; let id = repo . blob (& [5 , 4 , 6]) . unwrap () ; let blob = repo . find_blob (id) . unwrap () ; assert_eq ! (blob . id () , id) ; assert_eq ! (blob . size () , 3) ; assert_eq ! (blob . content () , [5 , 4 , 6]) ; assert ! (blob . is_binary ()) ; repo . find_object (id , None) . unwrap () . as_blob () . unwrap () ; repo . find_object (id , None) . unwrap () . into_blob () . ok () . unwrap () ; } # [test] fn path () { let td = TempDir :: new () . unwrap () ; let path = td . path () . join ("foo") ; File :: create (& path) . unwrap () . write_all (& [7 , 8 , 9]) . unwrap () ; let repo = Repository :: init (td . path ()) . unwrap () ; let id = repo . blob_path (& path) . unwrap () ; let blob = repo . find_blob (id) . unwrap () ; assert_eq ! (blob . content () , [7 , 8 , 9]) ; blob . into_object () ; } # [test] fn stream () { let td = TempDir :: new () . unwrap () ; let repo = Repository :: init (td . path ()) . unwrap () ; let mut ws = repo . blob_writer (Some (Path :: new ("foo"))) . unwrap () ; let wl = ws . write (& [10 , 11 , 12]) . unwrap () ; assert_eq ! (wl , 3) ; let id = ws . commit () . unwrap () ; let blob = repo . find_blob (id) . unwrap () ; assert_eq ! (blob . content () , [10 , 11 , 12]) ; blob . into_object () ; } }
};
}
