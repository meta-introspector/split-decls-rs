macro_rules! deps {
    () => {
        Connectivity!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < T , F > Connectivity < T , F > where T : FindExt + Exists , F : FnMut (& ObjectId , Kind) , { # [doc = " Instantiate a connectivity check."] pub fn new (db : T , missing_cb : F) -> Connectivity < T , F > { Connectivity { db , missing_cb , seen : HashSet :: default () , buf : Default :: default () , } } # [doc = " Run the connectivity check on the provided commit `oid`."] # [doc = ""] # [doc = " ### Algorithm"] # [doc = ""] # [doc = " Walk the trees and blobs referenced by the commit and verify they exist in the ODB."] # [doc = " Any objects previously encountered by this instance will be skipped silently."] # [doc = " Any referenced blobs that are not present in the ODB will result in a call to the  `missing_cb`."] # [doc = " Missing commits or trees will cause an error to be returned."] # [doc = "     - TODO: consider how to handle a missing commit (invoke `missing_cb`, or possibly return a Result?)"] pub fn check_commit (& mut self , oid : & ObjectId) -> Result < () , gix_object :: find :: existing_object :: Error > { if ! self . seen . insert (* oid) { return Ok (()) ; } let tree_id = { let commit = self . db . find_commit (oid , & mut self . buf) ? ; commit . tree () } ; let mut tree_ids = VecDeque :: from_iter (Some (tree_id)) ; while let Some (tree_id) = tree_ids . pop_front () { if self . seen . insert (tree_id) { self . check_tree (& tree_id , & mut tree_ids) ; } } Ok (()) } # [doc = " Blobs are checked right away, trees are stored in `tree_ids` for the parent to iterate them, and only"] # [doc = " if they have not been `seen` yet."] fn check_tree (& mut self , oid : & ObjectId , tree_ids : & mut VecDeque < ObjectId >) { let Ok (tree) = self . db . find_tree (oid , & mut self . buf) else { (self . missing_cb) (oid , Kind :: Tree) ; return ; } ; for entry_ref in tree . entries . iter () { match entry_ref . mode . kind () { EntryKind :: Tree => { let tree_id = entry_ref . oid . to_owned () ; tree_ids . push_back (tree_id) ; } EntryKind :: Blob | EntryKind :: BlobExecutable | EntryKind :: Link => { let blob_id = entry_ref . oid . to_owned () ; if self . seen . insert (blob_id) { check_blob (& self . db , & blob_id , & mut self . missing_cb) ; } } EntryKind :: Commit => { } } } } }
    };
}

impl_1!();