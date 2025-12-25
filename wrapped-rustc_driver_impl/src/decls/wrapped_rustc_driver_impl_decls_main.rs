use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn main() -> ! {
    let start_time = Instant::now();
    let start_rss = get_resident_set_size();
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    init_rustc_env_logger(&early_dcx);
    signal_handler::install();
    let mut callbacks = TimePassesCallbacks::default();
    install_ice_hook(DEFAULT_BUG_REPORT_URL, |_| ());
    install_ctrlc_handler();
    let exit_code = catch_with_exit_code(|| run_compiler(
        &args::raw_args(&early_dcx),
        &mut callbacks,
    ));
    if let Some(format) = callbacks.time_passes {
        let end_rss = get_resident_set_size();
        print_time_passes_entry(
            "total",
            start_time.elapsed(),
            start_rss,
            end_rss,
            format,
        );
    }
    process::exit(exit_code)
}
