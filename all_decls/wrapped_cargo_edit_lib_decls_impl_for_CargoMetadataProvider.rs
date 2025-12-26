use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl CargoMetadataProvider for MockCargoMetadataProvider {
    fn get_metadata(&self, _cargo_toml_path: &std::path::Path) -> Result<Box<dyn AnyMetadata>> {
        Ok(Box::new(DummyMetadata::default()))
    }
}
