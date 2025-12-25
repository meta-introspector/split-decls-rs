use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl GlobWalkerBuilder {
    /// Construct a new `GlobWalker` with a glob pattern.
    ///
    /// When iterated, the `base` directory will be recursively searched for paths
    /// matching `pattern`.
    pub fn new<P, S>(base: P, pattern: S) -> Self
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        GlobWalkerBuilder::from_patterns(base, &[pattern])
    }
    /// Construct a new `GlobWalker` from a list of patterns.
    ///
    /// When iterated, the `base` directory will be recursively searched for paths
    /// matching `patterns`.
    pub fn from_patterns<P, S>(base: P, patterns: &[S]) -> Self
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        fn normalize_pattern<S: AsRef<str>>(pattern: S) -> String {
            if pattern.as_ref() == "*" {
                String::from("/*")
            } else {
                pattern.as_ref().to_owned()
            }
        }
        GlobWalkerBuilder {
            root: base.as_ref().into(),
            patterns: patterns.iter().map(normalize_pattern).collect::<_>(),
            walker: WalkDir::new(base),
            case_insensitive: false,
            file_type: None,
        }
    }
    /// Set the minimum depth of entries yielded by the iterator.
    ///
    /// The smallest depth is `0` and always corresponds to the path given
    /// to the `new` function on this type. Its direct descendents have depth
    /// `1`, and their descendents have depth `2`, and so on.
    pub fn min_depth(mut self, depth: usize) -> Self {
        self.walker = self.walker.min_depth(depth);
        self
    }
    /// Set the maximum depth of entries yield by the iterator.
    ///
    /// The smallest depth is `0` and always corresponds to the path given
    /// to the `new` function on this type. Its direct descendents have depth
    /// `1`, and their descendents have depth `2`, and so on.
    ///
    /// Note that this will not simply filter the entries of the iterator, but
    /// it will actually avoid descending into directories when the depth is
    /// exceeded.
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.walker = self.walker.max_depth(depth);
        self
    }
    /// Follow symbolic links. By default, this is disabled.
    ///
    /// When `yes` is `true`, symbolic links are followed as if they were
    /// normal directories and files. If a symbolic link is broken or is
    /// involved in a loop, an error is yielded.
    ///
    /// When enabled, the yielded [`DirEntry`] values represent the target of
    /// the link while the path corresponds to the link. See the [`DirEntry`]
    /// type for more details.
    ///
    /// [`DirEntry`]: struct.DirEntry.html
    pub fn follow_links(mut self, yes: bool) -> Self {
        self.walker = self.walker.follow_links(yes);
        self
    }
    /// Set the maximum number of simultaneously open file descriptors used
    /// by the iterator.
    ///
    /// `n` must be greater than or equal to `1`. If `n` is `0`, then it is set
    /// to `1` automatically. If this is not set, then it defaults to some
    /// reasonably low number.
    ///
    /// This setting has no impact on the results yielded by the iterator
    /// (even when `n` is `1`). Instead, this setting represents a trade off
    /// between scarce resources (file descriptors) and memory. Namely, when
    /// the maximum number of file descriptors is reached and a new directory
    /// needs to be opened to continue iteration, then a previous directory
    /// handle is closed and has its unyielded entries stored in memory. In
    /// practice, this is a satisfying trade off because it scales with respect
    /// to the *depth* of your file tree. Therefore, low values (even `1`) are
    /// acceptable.
    ///
    /// Note that this value does not impact the number of system calls made by
    /// an exhausted iterator.
    ///
    /// # Platform behavior
    ///
    /// On Windows, if `follow_links` is enabled, then this limit is not
    /// respected. In particular, the maximum number of file descriptors opened
    /// is proportional to the depth of the directory tree traversed.
    pub fn max_open(mut self, n: usize) -> Self {
        self.walker = self.walker.max_open(n);
        self
    }
    /// Set a function for sorting directory entries.
    ///
    /// If a compare function is set, the resulting iterator will return all
    /// paths in sorted order. The compare function will be called to compare
    /// entries from the same directory.
    pub fn sort_by<F>(mut self, cmp: F) -> Self
    where
        F: FnMut(&DirEntry, &DirEntry) -> Ordering + Send + Sync + 'static,
    {
        self.walker = self.walker.sort_by(cmp);
        self
    }
    /// Yield a directory's contents before the directory itself. By default,
    /// this is disabled.
    ///
    /// When `yes` is `false` (as is the default), the directory is yielded
    /// before its contents are read. This is useful when, e.g. you want to
    /// skip processing of some directories.
    ///
    /// When `yes` is `true`, the iterator yields the contents of a directory
    /// before yielding the directory itself. This is useful when, e.g. you
    /// want to recursively delete a directory.
    pub fn contents_first(mut self, yes: bool) -> Self {
        self.walker = self.walker.contents_first(yes);
        self
    }
    /// Toggle whether the globs should be matched case insensitively or not.
    ///
    /// This is disabled by default.
    pub fn case_insensitive(mut self, yes: bool) -> Self {
        self.case_insensitive = yes;
        self
    }
    /// Toggle filtering by file type.
    /// `FileType` can be an OR of several types.
    ///
    /// Note that not all file-types can be whitelisted by this filter (e.g. char-devices, fifos, etc.)
    pub fn file_type(mut self, file_type: FileType) -> Self {
        self.file_type = Some(file_type);
        self
    }
    /// Finalize and build a `GlobWalker` instance.
    pub fn build(self) -> Result<GlobWalker, GlobError> {
        let mut builder = OverrideBuilder::new(self.root);
        builder.case_insensitive(self.case_insensitive).map_err(GlobError)?;
        for pattern in self.patterns {
            builder.add(pattern.as_ref()).map_err(GlobError)?;
        }
        Ok(GlobWalker {
            ignore: builder.build().map_err(GlobError)?,
            walker: self.walker.into_iter(),
            file_type_filter: self.file_type,
        })
    }
}
