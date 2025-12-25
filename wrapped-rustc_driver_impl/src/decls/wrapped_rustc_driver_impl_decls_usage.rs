use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn usage(verbose: bool, include_unstable_options: bool, nightly_build: bool) {
    let mut options = getopts::Options::new();
    for option in config::rustc_optgroups()
        .iter()
        .filter(|x| verbose || !x.is_verbose_help_only)
        .filter(|x| include_unstable_options || x.is_stable())
    {
        option.apply(&mut options);
    }
    let message = "Usage: rustc [OPTIONS] INPUT";
    let nightly_help = if nightly_build {
        "\n    -Z help             Print unstable compiler options"
    } else {
        ""
    };
    let verbose_help = if verbose {
        ""
    } else {
        "\n    --help -v           Print the full set of options rustc accepts"
    };
    let at_path = if verbose {
        "    @path               Read newline separated options from `path`\n"
    } else {
        ""
    };
    safe_println!(
        "{options}{at_path}\nAdditional help:
    -C help             Print codegen options
    -W help             \
              Print 'lint' options and default settings{nightly}{verbose}\n",
        options = options.usage(message), at_path = at_path, nightly = nightly_help,
        verbose = verbose_help
    );
}
