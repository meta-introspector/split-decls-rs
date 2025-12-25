use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn print_flag_list<T>(cmdline_opt: &str, flag_list: &[OptionDesc<T>]) {
    let max_len = flag_list
        .iter()
        .map(|opt_desc| opt_desc.name().chars().count())
        .max()
        .unwrap_or(0);
    for opt_desc in flag_list {
        safe_println!(
            "    {} {:>width$}=val -- {}", cmdline_opt, opt_desc.name().replace('_',
            "-"), opt_desc.desc(), width = max_len
        );
    }
}
