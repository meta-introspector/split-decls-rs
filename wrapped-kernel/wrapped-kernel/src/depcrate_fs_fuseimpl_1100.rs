// Generated macro for impl_1100 (impl)
macro_rules! Depcrate_fs_fuseimpl_1100 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1100"}
// Dependencies: {}
impl Read for FuseFileHandleInner { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { let mut len = buf . len () ; if len > MAX_READ_LEN { debug ! ("Reading longer than max_read_len: {len}") ; len = MAX_READ_LEN ; } if let (Some (nid) , Some (fh)) = (self . fuse_nid , self . fuse_fh) { let (cmd , rsp_payload_len) = ops :: Read :: create (nid , fh , len . try_into () . unwrap () , self . offset as u64) ; let rsp = get_filesystem_driver () . ok_or (Errno :: Nosys) ? . lock () . send_command (cmd , rsp_payload_len) ? ; let len : usize = if (rsp . headers . out_header . len as usize) - mem :: size_of :: < fuse_out_header > () >= len { len } else { (rsp . headers . out_header . len as usize) - mem :: size_of :: < fuse_out_header > () } ; self . offset += len ; buf [.. len] . copy_from_slice (& rsp . payload . unwrap () [.. len]) ; Ok (len) } else { debug ! ("File not open, cannot read!") ; Err (Errno :: Noent) } } }
};
}
