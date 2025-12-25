use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn dump_feature_usage_metrics(tcxt: TyCtxt<'_>, metrics_dir: &Path) {
    let hash = tcxt.crate_hash(LOCAL_CRATE);
    let crate_name = tcxt.crate_name(LOCAL_CRATE);
    let metrics_file_name = format!(
        "unstable_feature_usage_metrics-{crate_name}-{hash}.json"
    );
    let metrics_path = metrics_dir.join(metrics_file_name);
    if let Err(error) = tcxt.features().dump_feature_usage_metrics(metrics_path) {
        tcxt.dcx().emit_err(UnstableFeatureUsage { error });
    }
}
