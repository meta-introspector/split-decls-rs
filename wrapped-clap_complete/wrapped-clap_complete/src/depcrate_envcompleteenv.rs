// Generated macro for CompleteEnv (struct)
macro_rules! Depcrate_envCompleteEnv {
() => {
// Module: crate::env
// Provides: {"CompleteEnv"}
// Dependencies: {}
# [doc = " Environment-activated completions for your CLI"] # [doc = ""] # [doc = " Benefits over a CLI completion argument or subcommand"] # [doc = " - Performance: we don't need to generate [`clap::Command`] twice or parse arguments"] # [doc = " - Flexibility: there is no concern over it interfering with other CLI logic"] # [doc = ""] # [doc = " **Warning:** `stdout` should not be written to before [`CompleteEnv::complete`] has had a"] # [doc = " chance to run."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_complete::CompleteEnv;"] # [doc = " fn cli() -> clap::Command {"] # [doc = "     // ..."] # [doc = " #   clap::Command::new(\"empty\")"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     CompleteEnv::with_factory(cli)"] # [doc = "         .complete()"] # [doc = ""] # [doc = "     // ... rest of application logic"] # [doc = " }"] # [doc = " ```"] pub struct CompleteEnv < 's , F > { factory : F , var : & 'static str , bin : Option < String > , completer : Option < String > , shells : Shells < 's > , }
};
}
