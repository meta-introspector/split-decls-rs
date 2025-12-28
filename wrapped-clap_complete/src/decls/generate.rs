macro_rules! deps {
    () => {
        Generator!();
    };
}

macro_rules! generate {
    () => {
        deps!();
        # [doc = " Generate a completions file for a specified shell at runtime."] # [doc = ""] # [doc = " Until `cargo install` can install extra files like a completion script, this may be"] # [doc = " used e.g. in a command that outputs the contents of the completion script, to be"] # [doc = " redirected into a file by the user."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Assuming a separate `cli.rs` like the [`generate_to` example](generate_to()),"] # [doc = " we can let users generate a completion script using a command:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " // src/main.rs"] # [doc = ""] # [doc = " mod cli;"] # [doc = " use std::io;"] # [doc = " use clap_complete::{generate, shells::Bash};"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let matches = cli::build_cli().get_matches();"] # [doc = ""] # [doc = "     if matches.is_present(\"generate-bash-completions\") {"] # [doc = "         generate(Bash, &mut cli::build_cli(), \"myapp\", &mut io::stdout());"] # [doc = "     }"] # [doc = ""] # [doc = "     // normal logic continues..."] # [doc = " }"] # [doc = ""] # [doc = " ```"] # [doc = ""] # [doc = " Usage:"] # [doc = ""] # [doc = " ```console"] # [doc = " $ myapp generate-bash-completions > /usr/share/bash-completion/completions/myapp.bash"] # [doc = " ```"] pub fn generate < G , S > (generator : G , cmd : & mut Command , bin_name : S , buf : & mut dyn Write) where G : Generator , S : Into < String > , { cmd . set_bin_name (bin_name) ; _generate :: < G > (generator , cmd , buf) ; }
    };
}

generate!()