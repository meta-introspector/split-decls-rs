use std::time::Instant;
use rustc_errors::EarlyDiagCtxt;
use rustc_session::config::ErrorOutputType;

fn test_missing_deps() {
    let start = Instant::now();
    let diag = EarlyDiagCtxt::new();
    let output = ErrorOutputType::default();
}
