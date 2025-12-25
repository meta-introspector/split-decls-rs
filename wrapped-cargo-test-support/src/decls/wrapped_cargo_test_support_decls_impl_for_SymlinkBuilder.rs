use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl SymlinkBuilder {
    pub fn new(dst: PathBuf, src: PathBuf) -> SymlinkBuilder {
        SymlinkBuilder {
            dst,
            src,
            src_is_dir: false,
        }
    }
    pub fn new_dir(dst: PathBuf, src: PathBuf) -> SymlinkBuilder {
        SymlinkBuilder {
            dst,
            src,
            src_is_dir: true,
        }
    }
    #[cfg(unix)]
    fn mk(&self) {
        self.dirname().mkdir_p();
        t!(os::unix::fs::symlink(&self.dst, &self.src));
    }
    #[cfg(windows)]
    fn mk(&mut self) {
        self.dirname().mkdir_p();
        if self.src_is_dir {
            t!(os::windows::fs::symlink_dir(&self.dst, &self.src));
        } else {
            if let Some(ext) = self.dst.extension() {
                if ext == env::consts::EXE_EXTENSION {
                    self.src.set_extension(ext);
                }
            }
            t!(os::windows::fs::symlink_file(&self.dst, &self.src));
        }
    }
    fn dirname(&self) -> &Path {
        self.src.parent().unwrap()
    }
}
