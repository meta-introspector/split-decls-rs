use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn check_command(command_path: &Path, args: &[&str]) -> bool {
    let mut command = Command::new(command_path);
    let command_name = command.get_program().to_str().unwrap().to_owned();
    command.args(args);
    let output = match command.output() {
        Ok(output) => output,
        Err(e) => {
            if is_ci() && !matches!(command_name.as_str(), "hg" | "lldb") {
                panic!("expected command `{command_name}` to be somewhere in PATH: {e}",);
            }
            return false;
        }
    };
    if !output.status.success() {
        panic!(
            "expected command `{command_name}` to be runnable, got error {}:\n\
            stderr:{}\n\
            stdout:{}\n",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
    }
    true
}
