use crate::track_transform;
use crate::transform_safe::safe_string_transform;

/// Fix jobserver import references (safe string transformation)
pub fn fix_jobserver_imports(content: &str) -> String {
    safe_string_transform(content, "JOBSERVER_IMPORTS", "Fix jobserver import references", |content: &str| {
        content
            .replace("jobserver::", "jobserver_crate::")
            .replace("use jobserver;", "use jobserver_crate as jobserver;")
    })
}
