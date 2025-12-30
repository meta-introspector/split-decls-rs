// Generated macro for impl_1619 (impl)
macro_rules! Depcrate_shims_windows_fsimpl_1619 {
() => {
// Module: crate::shims::windows::fs
// Provides: {"impl_1619"}
// Dependencies: {}
impl FileAttributes { fn new < 'tcx > (mut value : u32 , ecx : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , FileAttributes > { let file_attribute_normal = ecx . eval_windows_u32 ("c" , "FILE_ATTRIBUTE_NORMAL") ; let file_flag_backup_semantics = ecx . eval_windows_u32 ("c" , "FILE_FLAG_BACKUP_SEMANTICS") ; let file_flag_open_reparse_point = ecx . eval_windows_u32 ("c" , "FILE_FLAG_OPEN_REPARSE_POINT") ; let mut out = FileAttributes :: ZERO ; if value & file_flag_backup_semantics != 0 { value &= ! file_flag_backup_semantics ; out |= FileAttributes :: BACKUP_SEMANTICS ; } if value & file_flag_open_reparse_point != 0 { value &= ! file_flag_open_reparse_point ; out |= FileAttributes :: OPEN_REPARSE ; } if value & file_attribute_normal != 0 { value &= ! file_attribute_normal ; out |= FileAttributes :: NORMAL ; } if value != 0 { throw_unsup_format ! ("CreateFileW: Unsupported flags_and_attributes: {value}") ; } if out == FileAttributes :: ZERO { out = FileAttributes :: NORMAL ; } interp_ok (out) } }
};
}
