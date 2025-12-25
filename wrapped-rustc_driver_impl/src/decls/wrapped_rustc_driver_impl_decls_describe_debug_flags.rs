use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn describe_debug_flags() {
    safe_println!("\nAvailable options:\n");
    print_flag_list("-Z", config::Z_OPTIONS);
}
