use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn parse(db: &dyn RootQueryDb, file_id: EditionedFileId) -> Parse<ast::SourceFile> {
    let _p = tracing::info_span!("parse", ? file_id).entered();
    let (file_id, edition) = file_id.unpack(db.as_dyn_database());
    let text = db.file_text(file_id).text(db);
    ast::SourceFile::parse(text, edition)
}
