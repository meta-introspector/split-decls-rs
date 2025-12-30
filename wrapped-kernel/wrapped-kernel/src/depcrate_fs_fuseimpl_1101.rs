// Generated macro for impl_1101 (impl)
macro_rules! Depcrate_fs_fuseimpl_1101 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1101"}
// Dependencies: {}
impl Write for FuseFileHandleInner { fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { debug ! ("FUSE write!") ; let mut truncated_len = buf . len () ; if truncated_len > MAX_WRITE_LEN { debug ! ("Writing longer than max_write_len: {} > {}" , buf . len () , MAX_WRITE_LEN) ; truncated_len = MAX_WRITE_LEN ; } if let (Some (nid) , Some (fh)) = (self . fuse_nid , self . fuse_fh) { let truncated_buf = Box :: < [u8] > :: from (& buf [.. truncated_len]) ; let (cmd , rsp_payload_len) = ops :: Write :: create (nid , fh , truncated_buf , self . offset as u64) ; let rsp = get_filesystem_driver () . ok_or (Errno :: Nosys) ? . lock () . send_command (cmd , rsp_payload_len) ? ; if rsp . headers . out_header . error < 0 { return Err (Errno :: Io) ; } let rsp_size = rsp . headers . op_header . size ; let rsp_len : usize = if rsp_size > u32 :: try_from (truncated_len) . unwrap () { truncated_len } else { rsp_size . try_into () . unwrap () } ; self . offset += rsp_len ; Ok (rsp_len) } else { warn ! ("File not open, cannot read!") ; Err (Errno :: Noent) } } fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
