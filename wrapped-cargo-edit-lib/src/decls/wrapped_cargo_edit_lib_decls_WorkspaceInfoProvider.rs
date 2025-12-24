use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub trait WorkspaceInfoProvider: Send + Sync {
    fn parse_members_file(
        &self,
        git_adapter: &dyn git_wrapper_lib::git_adapters::GitAdapter,
        cargo_metadata_provider: &dyn CargoMetadataProvider,
        project_root: &std::path::Path,
    ) -> anyhow::Result<Vec<WorkspaceInfo>>;
}
