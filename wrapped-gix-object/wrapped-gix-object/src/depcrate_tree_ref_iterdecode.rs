// Generated macro for decode (module)
macro_rules! Depcrate_tree_ref_iterdecode {
() => {
// Module: crate::tree::ref_iter
// Provides: {"decode"}
// Dependencies: {}
mod decode { use bstr :: ByteSlice ; use winnow :: { error :: ParserError , prelude :: * } ; use crate :: { tree , tree :: EntryRef , TreeRef } ; pub fn fast_entry (i : & [u8]) -> Option < (& [u8] , EntryRef < '_ >) > { let (mode , i) = tree :: EntryMode :: extract_from_bytes (i) ? ; let (filename , i) = i . split_at (i . find_byte (0) ?) ; let i = & i [1 ..] ; const HASH_LEN_FIXME : usize = 20 ; let (oid , i) = match i . len () { len if len < HASH_LEN_FIXME => return None , _ => i . split_at (20) , } ; Some ((i , EntryRef { mode , filename : filename . as_bstr () , oid : gix_hash :: oid :: try_from_bytes (oid) . expect ("we counted exactly 20 bytes") , } ,)) } pub fn tree < 'a , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8]) -> ModalResult < TreeRef < 'a > , E > { let mut out = Vec :: new () ; let mut i = & * * i ; while ! i . is_empty () { let Some ((rest , entry)) = fast_entry (i) else { # [allow (clippy :: unit_arg)] return Err (winnow :: error :: ErrMode :: from_input (& i)) ; } ; i = rest ; out . push (entry) ; } Ok (TreeRef { entries : out }) } }
};
}
