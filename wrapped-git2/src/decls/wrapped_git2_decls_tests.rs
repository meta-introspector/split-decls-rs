use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    use super::{FileMode, ObjectType};
    #[test]
    fn convert() {
        assert_eq!(ObjectType::Blob.str(), "blob");
        assert_eq!(ObjectType::from_str("blob"), Some(ObjectType::Blob));
        assert!(ObjectType::Blob.is_loose());
    }
    #[test]
    fn convert_filemode() {
        assert_eq!(i32::from(FileMode::Blob), 0o100644);
        assert_eq!(i32::from(FileMode::BlobGroupWritable), 0o100664);
        assert_eq!(i32::from(FileMode::BlobExecutable), 0o100755);
        assert_eq!(u32::from(FileMode::Blob), 0o100644);
        assert_eq!(u32::from(FileMode::BlobGroupWritable), 0o100664);
        assert_eq!(u32::from(FileMode::BlobExecutable), 0o100755);
    }
    #[test]
    fn bitflags_partial_eq() {
        use super::{
            AttrCheckFlags, CheckoutNotificationType, CredentialType, DiffFlags, DiffStatsFormat,
            IndexAddOption, IndexEntryExtendedFlag, IndexEntryFlag, MergeAnalysis, MergePreference,
            OdbLookupFlags, PathspecFlags, ReferenceFormat, RepositoryInitMode,
            RepositoryOpenFlags, RevparseMode, Sort, StashApplyFlags, StashFlags, Status,
            SubmoduleStatus,
        };
        assert_eq!(
            AttrCheckFlags::FILE_THEN_INDEX,
            AttrCheckFlags::FILE_THEN_INDEX
        );
        assert_eq!(
            CheckoutNotificationType::CONFLICT,
            CheckoutNotificationType::CONFLICT
        );
        assert_eq!(
            CredentialType::USER_PASS_PLAINTEXT,
            CredentialType::USER_PASS_PLAINTEXT
        );
        assert_eq!(DiffFlags::BINARY, DiffFlags::BINARY);
        assert_eq!(
            DiffStatsFormat::INCLUDE_SUMMARY,
            DiffStatsFormat::INCLUDE_SUMMARY
        );
        assert_eq!(
            IndexAddOption::CHECK_PATHSPEC,
            IndexAddOption::CHECK_PATHSPEC
        );
        assert_eq!(
            IndexEntryExtendedFlag::INTENT_TO_ADD,
            IndexEntryExtendedFlag::INTENT_TO_ADD
        );
        assert_eq!(IndexEntryFlag::EXTENDED, IndexEntryFlag::EXTENDED);
        assert_eq!(
            MergeAnalysis::ANALYSIS_FASTFORWARD,
            MergeAnalysis::ANALYSIS_FASTFORWARD
        );
        assert_eq!(
            MergePreference::FASTFORWARD_ONLY,
            MergePreference::FASTFORWARD_ONLY
        );
        assert_eq!(OdbLookupFlags::NO_REFRESH, OdbLookupFlags::NO_REFRESH);
        assert_eq!(PathspecFlags::FAILURES_ONLY, PathspecFlags::FAILURES_ONLY);
        assert_eq!(
            ReferenceFormat::ALLOW_ONELEVEL,
            ReferenceFormat::ALLOW_ONELEVEL
        );
        assert_eq!(
            RepositoryInitMode::SHARED_ALL,
            RepositoryInitMode::SHARED_ALL
        );
        assert_eq!(RepositoryOpenFlags::CROSS_FS, RepositoryOpenFlags::CROSS_FS);
        assert_eq!(RevparseMode::RANGE, RevparseMode::RANGE);
        assert_eq!(Sort::REVERSE, Sort::REVERSE);
        assert_eq!(
            StashApplyFlags::REINSTATE_INDEX,
            StashApplyFlags::REINSTATE_INDEX
        );
        assert_eq!(StashFlags::INCLUDE_IGNORED, StashFlags::INCLUDE_IGNORED);
        assert_eq!(Status::WT_MODIFIED, Status::WT_MODIFIED);
        assert_eq!(SubmoduleStatus::WD_ADDED, SubmoduleStatus::WD_ADDED);
    }
}
