// Generated macro for impl_11 (impl)
macro_rules! Depcrate_errorimpl_11 {
() => {
// Module: crate::error
// Provides: {"impl_11"}
// Dependencies: {}
impl Error { # [must_use] pub fn new (result_code : c_int) -> Self { let code = match result_code & 0xff { super :: SQLITE_INTERNAL => ErrorCode :: InternalMalfunction , super :: SQLITE_PERM => ErrorCode :: PermissionDenied , super :: SQLITE_ABORT => ErrorCode :: OperationAborted , super :: SQLITE_BUSY => ErrorCode :: DatabaseBusy , super :: SQLITE_LOCKED => ErrorCode :: DatabaseLocked , super :: SQLITE_NOMEM => ErrorCode :: OutOfMemory , super :: SQLITE_READONLY => ErrorCode :: ReadOnly , super :: SQLITE_INTERRUPT => ErrorCode :: OperationInterrupted , super :: SQLITE_IOERR => ErrorCode :: SystemIoFailure , super :: SQLITE_CORRUPT => ErrorCode :: DatabaseCorrupt , super :: SQLITE_NOTFOUND => ErrorCode :: NotFound , super :: SQLITE_FULL => ErrorCode :: DiskFull , super :: SQLITE_CANTOPEN => ErrorCode :: CannotOpen , super :: SQLITE_PROTOCOL => ErrorCode :: FileLockingProtocolFailed , super :: SQLITE_SCHEMA => ErrorCode :: SchemaChanged , super :: SQLITE_TOOBIG => ErrorCode :: TooBig , super :: SQLITE_CONSTRAINT => ErrorCode :: ConstraintViolation , super :: SQLITE_MISMATCH => ErrorCode :: TypeMismatch , super :: SQLITE_MISUSE => ErrorCode :: ApiMisuse , super :: SQLITE_NOLFS => ErrorCode :: NoLargeFileSupport , super :: SQLITE_AUTH => ErrorCode :: AuthorizationForStatementDenied , super :: SQLITE_RANGE => ErrorCode :: ParameterOutOfRange , super :: SQLITE_NOTADB => ErrorCode :: NotADatabase , _ => ErrorCode :: Unknown , } ; Self { code , extended_code : result_code , } } }
};
}
