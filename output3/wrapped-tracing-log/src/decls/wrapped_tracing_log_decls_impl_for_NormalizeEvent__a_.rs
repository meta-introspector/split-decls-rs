use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> NormalizeEvent<'a> for Event<'a> {
    fn normalized_metadata(&'a self) -> Option<Metadata<'a>> {
        let original = self.metadata();
        if self.is_log() {
            let mut fields = LogVisitor::new_for(self, level_to_cs(*original.level()).1);
            self.record(&mut fields);
            Some(Metadata::new(
                "log event",
                fields.target.unwrap_or("log"),
                *original.level(),
                fields.file,
                fields.line.map(|l| l as u32),
                fields.module_path,
                field::FieldSet::new(&["message"], original.callsite()),
                Kind::EVENT,
            ))
        } else {
            None
        }
    }
    fn is_log(&self) -> bool {
        self.metadata().callsite() == identify_callsite!(level_to_cs(*self.metadata().level()).0)
    }
}
