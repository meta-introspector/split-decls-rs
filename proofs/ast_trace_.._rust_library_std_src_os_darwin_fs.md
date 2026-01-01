# AST Trace: ../rust/library/std/src/os/darwin/fs.rs

Generated 6 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
//! Darwin-specific extension traits to [`fs`].
//!
//! [`fs`]: crate::fs
#![stable(feature = "metadata_ext", since = "1.1.0")]

use crate::fs::{self, Metadata};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::sealed::Sealed;
use crate::sys_common::{AsInner, AsInnerMut, IntoInner};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=as_raw_stat | COMPLEXITY=8 | LINES=71

```rust
use crate::time::SystemTime;

/// OS-specific extensions to [`fs::Metadata`].
///
/// [`fs::Metadata`]: crate::fs::Metadata
#[stable(feature = "metadata_ext", since = "1.1.0")]
pub trait MetadataExt {
    /// Gain a reference to the underlying `stat` structure which contains
    /// the raw information returned by the OS.
    ///
    /// The contents of the returned `stat` are **not** consistent across
    /// Unix platforms. The `os::unix::fs::MetadataExt` trait contains the
    /// cross-Unix abstractions contained within the raw stat.
    #[stable(feature = "metadata_ext", since = "1.1.0")]
    #[deprecated(
        since = "1.8.0",
        note = "deprecated in favor of the accessor \
                methods of this trait"
    )]
    #[allow(deprecated)]
    // Only available on macOS and iOS, since they were stably exposed there.
    #[cfg(any(doc, target_os = "macos", target_os = "ios"))]
    #[doc(cfg(any(target_os = "macos", target_os = "ios")))]
    fn as_raw_stat(&self) -> &super::raw::stat;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_dev(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_ino(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_mode(&self) -> u32;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_nlink(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_uid(&self) -> u32;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_gid(&self) -> u32;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_rdev(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_size(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_atime(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_atime_nsec(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_mtime(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_mtime_nsec(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_ctime(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_ctime_nsec(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_birthtime(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_birthtime_nsec(&self) -> i64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_blksize(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_blocks(&self) -> u64;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_flags(&self) -> u32;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_gen(&self) -> u32;
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_lspare(&self) -> u32;
    #[cfg(target_os = "macos")]
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_qspare(&self) -> [u64; 2];
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=as_raw_stat | COMPLEXITY=36 | LINES=77

```rust
#[stable(feature = "metadata_ext", since = "1.1.0")]
impl MetadataExt for Metadata {
    #[allow(deprecated)]
    #[cfg(any(doc, target_os = "macos", target_os = "ios"))]
    fn as_raw_stat(&self) -> &super::raw::stat {
        unsafe { &*(self.as_inner().as_inner() as *const libc::stat as *const super::raw::stat) }
    }
    fn st_dev(&self) -> u64 {
        self.as_inner().as_inner().st_dev as u64
    }
    fn st_ino(&self) -> u64 {
        self.as_inner().as_inner().st_ino as u64
    }
    fn st_mode(&self) -> u32 {
        self.as_inner().as_inner().st_mode as u32
    }
    fn st_nlink(&self) -> u64 {
        self.as_inner().as_inner().st_nlink as u64
    }
    fn st_uid(&self) -> u32 {
        self.as_inner().as_inner().st_uid as u32
    }
    fn st_gid(&self) -> u32 {
        self.as_inner().as_inner().st_gid as u32
    }
    fn st_rdev(&self) -> u64 {
        self.as_inner().as_inner().st_rdev as u64
    }
    fn st_size(&self) -> u64 {
        self.as_inner().as_inner().st_size as u64
    }
    fn st_atime(&self) -> i64 {
        self.as_inner().as_inner().st_atime as i64
    }
    fn st_atime_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_atime_nsec as i64
    }
    fn st_mtime(&self) -> i64 {
        self.as_inner().as_inner().st_mtime as i64
    }
    fn st_mtime_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_mtime_nsec as i64
    }
    fn st_ctime(&self) -> i64 {
        self.as_inner().as_inner().st_ctime as i64
    }
    fn st_ctime_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_ctime_nsec as i64
    }
    fn st_birthtime(&self) -> i64 {
        self.as_inner().as_inner().st_birthtime as i64
    }
    fn st_birthtime_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_birthtime_nsec as i64
    }
    fn st_blksize(&self) -> u64 {
        self.as_inner().as_inner().st_blksize as u64
    }
    fn st_blocks(&self) -> u64 {
        self.as_inner().as_inner().st_blocks as u64
    }
    fn st_gen(&self) -> u32 {
        self.as_inner().as_inner().st_gen as u32
    }
    fn st_flags(&self) -> u32 {
        self.as_inner().as_inner().st_flags as u32
    }
    fn st_lspare(&self) -> u32 {
        self.as_inner().as_inner().st_lspare as u32
    }
    #[cfg(target_os = "macos")]
    fn st_qspare(&self) -> [u64; 2] {
        let qspare = self.as_inner().as_inner().st_qspare;
        [qspare[0] as u64, qspare[1] as u64]
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=set_created | COMPLEXITY=2 | LINES=8

```rust
/// OS-specific extensions to [`fs::FileTimes`].
#[stable(feature = "file_set_times", since = "1.75.0")]
pub trait FileTimesExt: Sealed {
    /// Set the creation time of a file.
    #[stable(feature = "file_set_times", since = "1.75.0")]
    fn set_created(self, t: SystemTime) -> Self;
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=set_created | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "file_set_times", since = "1.75.0")]
impl FileTimesExt for fs::FileTimes {
    fn set_created(mut self, t: SystemTime) -> Self {
        self.as_inner_mut().set_created(t.into_inner());
        self
    }
}
```

---
*Generated by AST tracing system*
