macro_rules! UnstableFeatures {
    () => {
        # [derive (Clone , Copy , Debug , Hash)] pub enum UnstableFeatures { # [doc = " Disallow use of unstable features, as on beta/stable channels."] Disallow , # [doc = " Allow use of unstable features, as on nightly."] Allow , # [doc = " Errors are bypassed for bootstrapping. This is required any time"] # [doc = " during the build that feature-related lints are set to warn or above"] # [doc = " because the build turns on warnings-as-errors and uses lots of unstable"] # [doc = " features. As a result, this is always required for building Rust itself."] Cheat , }
    };
}

UnstableFeatures!()