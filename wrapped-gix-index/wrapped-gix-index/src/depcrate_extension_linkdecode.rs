// Generated macro for decode (function)
macro_rules! Depcrate_extension_linkdecode {
() => {
// Module: crate::extension::link
// Provides: {"decode"}
// Dependencies: {}
pub (crate) fn decode (data : & [u8] , object_hash : gix_hash :: Kind) -> Result < Link , decode :: Error > { let (id , data) = data . split_at_checked (object_hash . len_in_bytes ()) . ok_or (decode :: Error :: Corrupt ("link extension too short to read share index checksum" ,)) . map (| (id , d) | (gix_hash :: ObjectId :: from_bytes_or_panic (id) , d)) ? ; if data . is_empty () { return Ok (Link { shared_index_checksum : id , bitmaps : None , }) ; } let (delete , data) = gix_bitmap :: ewah :: decode (data) . map_err (| err | decode :: Error :: BitmapDecode { kind : "delete" , err }) ? ; let (replace , data) = gix_bitmap :: ewah :: decode (data) . map_err (| err | decode :: Error :: BitmapDecode { kind : "replace" , err }) ? ; if ! data . is_empty () { return Err (decode :: Error :: Corrupt ("garbage trailing link extension")) ; } Ok (Link { shared_index_checksum : id , bitmaps : Some (Bitmaps { delete , replace }) , }) }
};
}
