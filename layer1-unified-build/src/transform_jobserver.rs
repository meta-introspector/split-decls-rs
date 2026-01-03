use crate::track_transform;

/// Fix jobserver import references
pub fn fix_jobserver_imports(content: &str) -> String {
    track_transform!(content, "JOBSERVER_IMPORTS", "Fix jobserver import references", |content: &str| {
        content
            .replace("jobserver::", "jobserver_crate::")
            .replace("use jobserver;", "use jobserver_crate as jobserver;")
    })
}
