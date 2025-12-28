macro_rules! deps {
    () => {
        Section!();
        Key!();
        Core!();
    };
}

macro_rules! impl_609 {
    () => {
        deps!();
        impl Section for Core { fn name (& self) -> & str { "core" } fn keys (& self) -> & [& dyn Key] { & [& Self :: ABBREV , & Self :: BARE , & Self :: BIG_FILE_THRESHOLD , & Self :: CHECK_STAT , & Self :: DELTA_BASE_CACHE_LIMIT , & Self :: DISAMBIGUATE , & Self :: EDITOR , & Self :: FILE_MODE , & Self :: IGNORE_CASE , & Self :: FILES_REF_LOCK_TIMEOUT , & Self :: PACKED_REFS_TIMEOUT , & Self :: MULTIPACK_INDEX , & Self :: LOG_ALL_REF_UPDATES , & Self :: PRECOMPOSE_UNICODE , & Self :: REPOSITORY_FORMAT_VERSION , & Self :: SYMLINKS , & Self :: TRUST_C_TIME , & Self :: WORKTREE , & Self :: PROTECT_HFS , & Self :: PROTECT_NTFS , & Self :: ASKPASS , & Self :: EXCLUDES_FILE , & Self :: ATTRIBUTES_FILE , & Self :: SSH_COMMAND , & Self :: USE_REPLACE_REFS , & Self :: COMMIT_GRAPH , # [cfg (feature = "attributes")] & Self :: SAFE_CRLF , # [cfg (feature = "attributes")] & Self :: AUTO_CRLF , # [cfg (feature = "attributes")] & Self :: EOL , # [cfg (feature = "attributes")] & Self :: CHECK_ROUND_TRIP_ENCODING ,] } }
    };
}

impl_609!();