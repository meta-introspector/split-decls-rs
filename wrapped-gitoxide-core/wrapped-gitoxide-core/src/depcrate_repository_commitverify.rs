// Generated macro for verify (function)
macro_rules! Depcrate_repository_commitverify {
() => {
// Module: crate::repository::commit
// Provides: {"verify"}
// Dependencies: {}
# [doc = " Note that this is a quick implementation of commit signature verification that ignores a lot of what"] # [doc = " git does and can do, while focussing on the gist of it."] # [doc = " For this to go into `gix`, one will have to implement many more options and various validation programs."] pub fn verify (repo : gix :: Repository , rev_spec : Option < & str >) -> Result < () > { let rev_spec = rev_spec . unwrap_or ("HEAD") ; let commit = repo . rev_parse_single (format ! ("{rev_spec}^{{commit}}") . as_str ()) ? . object () ? . into_commit () ; let (signature , signed_data) = commit . signature () . context ("Could not parse commit to obtain signature") ? . ok_or_else (| | anyhow ! ("Commit at {rev_spec} is not signed")) ? ; let mut signature_storage = tempfile :: NamedTempFile :: new () ? ; signature_storage . write_all (signature . as_ref ()) ? ; let signed_storage = signature_storage . into_temp_path () ; let mut cmd : std :: process :: Command = gix :: command :: prepare ("gpg") . into () ; cmd . args (["--keyid-format=long" , "--status-fd=1" , "--verify"]) . arg (& signed_storage) . arg ("-") . stdin (Stdio :: piped ()) ; gix :: trace :: debug ! ("About to execute {cmd:?}") ; let mut child = cmd . spawn () ? ; child . stdin . take () . expect ("configured") . write_all (signed_data . to_bstring () . as_ref ()) ? ; if ! child . wait () ? . success () { bail ! ("Command {cmd:?} failed") ; } Ok (()) }
};
}
