macro_rules! crate_name {
    () => {
        # [doc = " Allows you to pull the name from your Cargo.toml at compile time."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** This macro extracts the name from an environment variable `CARGO_PKG_NAME`."] # [doc = " When the crate name is set to something different from the package name,"] # [doc = " use environment variables `CARGO_CRATE_NAME` or `CARGO_BIN_NAME`."] # [doc = " See [the Cargo Book](https://doc.rust-lang.org/cargo/reference/environment-variables.html)"] # [doc = " for more information."] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::crate_name;"] # [doc = " # use clap::Command;"] # [doc = " let m = Command::new(crate_name!())"] # [doc = "             .get_matches();"] # [doc = " ```"] # [cfg (feature = "cargo")] # [macro_export] macro_rules ! crate_name { () => { env ! ("CARGO_PKG_NAME") } ; }
    };
}

crate_name!();