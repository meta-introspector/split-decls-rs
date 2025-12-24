use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn relevant_crates(db: &dyn RootQueryDb, file_id: FileId) -> Arc<[Crate]> {
    let _p = tracing::info_span!("relevant_crates").entered();
    let source_root = db.file_source_root(file_id);
    db.source_root_crates(source_root.source_root_id(db))
}
