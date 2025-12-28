macro_rules! MINIMUM_TOOLCHAIN_VERSION_SUPPORTING_LOCKFILE_PATH {
    () => {
        pub (crate) const MINIMUM_TOOLCHAIN_VERSION_SUPPORTING_LOCKFILE_PATH : semver :: Version = semver :: Version { major : 1 , minor : 82 , patch : 0 , pre : semver :: Prerelease :: EMPTY , build : semver :: BuildMetadata :: EMPTY , } ;
    };
}

MINIMUM_TOOLCHAIN_VERSION_SUPPORTING_LOCKFILE_PATH!();