use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Default)]
pub struct Files {
    files: Arc<DashMap<vfs::FileId, FileText, BuildHasherDefault<FxHasher>>>,
    source_roots: Arc<
        DashMap<SourceRootId, SourceRootInput, BuildHasherDefault<FxHasher>>,
    >,
    file_source_roots: Arc<
        DashMap<vfs::FileId, FileSourceRootInput, BuildHasherDefault<FxHasher>>,
    >,
}
