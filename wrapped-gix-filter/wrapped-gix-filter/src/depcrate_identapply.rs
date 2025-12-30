// Generated macro for apply (function)
macro_rules! Depcrate_identapply {
() => {
// Module: crate::ident
// Provides: {"apply"}
// Dependencies: {}
# [doc = " Substitute all occurrences of `$Id$` with `$Id: <hexsha-of-input>$` if present in `src` and write all changes to `buf`,"] # [doc = " with `object_hash` being used accordingly. Return `true` if `buf` was written to or `false` if no change was made"] # [doc = " (as there was nothing to do)."] # [doc = ""] # [doc = " ### Deviation"] # [doc = ""] # [doc = " `Git` also tries to cleanup 'stray' substituted `$Id: <hex>$`, but we don't do that, sticking exactly to what ought to be done."] # [doc = " The respective code is up to 16 years old and one might assume that `git` by now handles checking and checkout filters correctly."] pub fn apply (src : & [u8] , object_hash : gix_hash :: Kind , buf : & mut Vec < u8 >) -> Result < bool , apply :: Error > { const HASH_LEN : usize = ": " . len () + gix_hash :: Kind :: longest () . len_in_hex () ; let mut id = None ; let mut ofs = 0 ; while let Some (pos) = src [ofs ..] . find (b"$Id$") { let id = match id { None => { let new_id = gix_object :: compute_hash (object_hash , gix_object :: Kind :: Blob , src) ? ; id = new_id . into () ; clear_and_set_capacity (buf , src . len () + HASH_LEN) ? ; new_id } Some (id) => id . to_owned () , } ; buf . push_str (& src [ofs ..] [.. pos + 3]) ; buf . push_str (b": ") ; id . write_hex_to (& mut * buf) . expect ("writes to memory always work") ; buf . push (b'$') ; ofs += pos + 4 ; } if id . is_some () { buf . push_str (& src [ofs ..]) ; } Ok (id . is_some ()) }
};
}
