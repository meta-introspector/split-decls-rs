use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Version {
    /// Create `Version` with an empty pre-release and build metadata.
    ///
    /// Equivalent to:
    ///
    /// ```
    /// # use semver::{BuildMetadata, Prerelease, Version};
    /// #
    /// # const fn new(major: u64, minor: u64, patch: u64) -> Version {
    /// Version {
    ///     major,
    ///     minor,
    ///     patch,
    ///     pre: Prerelease::EMPTY,
    ///     build: BuildMetadata::EMPTY,
    /// }
    /// # }
    /// ```
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Version {
            major,
            minor,
            patch,
            pre: Prerelease::EMPTY,
            build: BuildMetadata::EMPTY,
        }
    }
    /// Create `Version` by parsing from string representation.
    ///
    /// # Errors
    ///
    /// Possible reasons for the parse to fail include:
    ///
    /// - `1.0` &mdash; too few numeric components. A SemVer version must have
    ///   exactly three. If you are looking at something that has fewer than
    ///   three numbers in it, it's possible it is a `VersionReq` instead (with
    ///   an implicit default `^` comparison operator).
    ///
    /// - `1.0.01` &mdash; a numeric component has a leading zero.
    ///
    /// - `1.0.unknown` &mdash; unexpected character in one of the components.
    ///
    /// - `1.0.0-` or `1.0.0+` &mdash; the pre-release or build metadata are
    ///   indicated present but empty.
    ///
    /// - `1.0.0-alpha_123` &mdash; pre-release or build metadata have something
    ///   outside the allowed characters, which are `0-9`, `A-Z`, `a-z`, `-`,
    ///   and `.` (dot).
    ///
    /// - `23456789999999999999.0.0` &mdash; overflow of a u64.
    pub fn parse(text: &str) -> Result<Self, Error> {
        Version::from_str(text)
    }
    /// Compare the major, minor, patch, and pre-release value of two versions,
    /// disregarding build metadata. Versions that differ only in build metadata
    /// are considered equal. This comparison is what the SemVer spec refers to
    /// as "precedence".
    ///
    /// # Example
    ///
    /// ```
    /// use semver::Version;
    ///
    /// let mut versions = [
    ///     "1.20.0+c144a98".parse::<Version>().unwrap(),
    ///     "1.20.0".parse().unwrap(),
    ///     "1.0.0".parse().unwrap(),
    ///     "1.0.0-alpha".parse().unwrap(),
    ///     "1.20.0+bc17664".parse().unwrap(),
    /// ];
    ///
    /// // This is a stable sort, so it preserves the relative order of equal
    /// // elements. The three 1.20.0 versions differ only in build metadata so
    /// // they are not reordered relative to one another.
    /// versions.sort_by(Version::cmp_precedence);
    /// assert_eq!(versions, [
    ///     "1.0.0-alpha".parse().unwrap(),
    ///     "1.0.0".parse().unwrap(),
    ///     "1.20.0+c144a98".parse().unwrap(),
    ///     "1.20.0".parse().unwrap(),
    ///     "1.20.0+bc17664".parse().unwrap(),
    /// ]);
    ///
    /// // Totally order the versions, including comparing the build metadata.
    /// versions.sort();
    /// assert_eq!(versions, [
    ///     "1.0.0-alpha".parse().unwrap(),
    ///     "1.0.0".parse().unwrap(),
    ///     "1.20.0".parse().unwrap(),
    ///     "1.20.0+bc17664".parse().unwrap(),
    ///     "1.20.0+c144a98".parse().unwrap(),
    /// ]);
    /// ```
    pub fn cmp_precedence(&self, other: &Self) -> Ordering {
        Ord::cmp(
            &(self.major, self.minor, self.patch, &self.pre),
            &(other.major, other.minor, other.patch, &other.pre),
        )
    }
}
