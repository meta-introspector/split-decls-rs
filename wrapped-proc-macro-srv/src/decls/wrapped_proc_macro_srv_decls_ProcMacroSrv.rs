use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct ProcMacroSrv<'env> {
    expanders: Mutex<HashMap<Utf8PathBuf, Arc<dylib::Expander>>>,
    env: &'env EnvSnapshot,
    temp_dir: TempDir,
}
