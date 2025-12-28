macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! write {
    () => {
        deps!();
        # [doc = ""] pub mod write { pub (crate) mod function { use std :: io :: Write ; use super :: Error ; use crate :: Update ; # [doc = " Write the [previously obtained](crate::read()) (possibly non-existing) `shallow_commits` to the shallow `file`"] # [doc = " after applying all `updates`."] # [doc = ""] # [doc = " If this leaves the list of shallow commits empty, the file is removed."] # [doc = ""] # [doc = " ### Deviation"] # [doc = ""] # [doc = " Git also prunes the set of shallow commits while writing, we don't until we support some sort of pruning."] pub fn write (mut file : gix_lock :: File , shallow_commits : Option < Vec < gix_hash :: ObjectId > > , updates : & [Update] ,) -> Result < () , Error > { let mut shallow_commits = shallow_commits . unwrap_or_default () ; for update in updates { match update { Update :: Shallow (id) => { shallow_commits . push (* id) ; } Update :: Unshallow (id) => shallow_commits . retain (| oid | oid != id) , } } if shallow_commits . is_empty () { std :: fs :: remove_file (file . resource_path ()) ? ; drop (file) ; return Ok (()) ; } if shallow_commits . is_empty () { if let Err (err) = std :: fs :: remove_file (file . resource_path ()) { if err . kind () != std :: io :: ErrorKind :: NotFound { return Err (err . into ()) ; } } } else { shallow_commits . sort () ; let mut buf = Vec :: < u8 > :: new () ; for commit in shallow_commits { commit . write_hex_to (& mut buf) . map_err (Error :: Io) ? ; buf . push (b'\n') ; } file . write_all (& buf) . map_err (Error :: Io) ? ; file . flush () ? ; } file . commit () ? ; Ok (()) } } # [doc = " The error returned by [`write()`](crate::write())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Commit (# [from] gix_lock :: commit :: Error < gix_lock :: File >) , # [error ("Could not remove an empty shallow file")] RemoveEmpty (# [from] std :: io :: Error) , # [error ("Failed to write object id to shallow file")] Io (std :: io :: Error) , } }
    };
}

write!();