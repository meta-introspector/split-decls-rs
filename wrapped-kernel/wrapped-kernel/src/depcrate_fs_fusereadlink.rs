// Generated macro for readlink (function)
macro_rules! Depcrate_fs_fusereadlink {
() => {
// Module: crate::fs::fuse
// Provides: {"readlink"}
// Dependencies: {}
fn readlink (nid : u64) -> io :: Result < String > { let len = MAX_READ_LEN as u32 ; let (cmd , rsp_payload_len) = ops :: Readlink :: create (nid , len) ; let rsp = get_filesystem_driver () . unwrap () . lock () . send_command (cmd , rsp_payload_len) ? ; let len : usize = if rsp . headers . out_header . len as usize - mem :: size_of :: < fuse_out_header > () >= usize :: try_from (len) . unwrap () { len . try_into () . unwrap () } else { (rsp . headers . out_header . len as usize) - mem :: size_of :: < fuse_out_header > () } ; Ok (String :: from_utf8 (rsp . payload . unwrap () [.. len] . to_vec ()) . unwrap ()) }
};
}
