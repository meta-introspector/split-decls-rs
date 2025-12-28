macro_rules! Config {
    () => {
        # [doc = " A structure representing a git configuration key/value store"] pub struct Config { raw : * mut raw :: git_config , }
    };
}

Config!();