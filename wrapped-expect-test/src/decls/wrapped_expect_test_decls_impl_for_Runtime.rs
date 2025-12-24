use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Runtime {
    fn fail_expect(expect: &Expect, expected: &str, actual: &str) {
        let mut rt = RT.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if update_expect() {
            println!("\x1b[1m\x1b[92mupdating\x1b[0m: {}", expect.position);
            rt.per_file
                .entry(expect.position.file)
                .or_insert_with(|| FileRuntime::new(expect))
                .update(expect, actual);
            return;
        }
        rt.panic(expect.position.to_string(), expected, actual);
    }
    fn fail_file(expect: &ExpectFile, expected: &str, actual: &str) {
        let mut rt = RT.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if update_expect() {
            println!("\x1b[1m\x1b[92mupdating\x1b[0m: {}", expect.path.display());
            expect.write(actual);
            return;
        }
        rt.panic(expect.path.display().to_string(), expected, actual);
    }
    fn panic(&mut self, position: String, expected: &str, actual: &str) {
        let print_help = !mem::replace(&mut self.help_printed, true);
        let help = if print_help { HELP } else { "" };
        let diff = dissimilar::diff(expected, actual);
        println!(
            "\n
\x1b[1m\x1b[91merror\x1b[97m: expect test failed\x1b[0m
   \x1b[1m\x1b[34m-->\x1b[0m {}
{}
\x1b[1mExpect\x1b[0m:
----
{}
----

\x1b[1mActual\x1b[0m:
----
{}
----

\x1b[1mDiff\x1b[0m:
----
{}
----
",
            position, help, expected, actual, format_chunks(diff)
        );
        panic::resume_unwind(Box::new(()));
    }
}
