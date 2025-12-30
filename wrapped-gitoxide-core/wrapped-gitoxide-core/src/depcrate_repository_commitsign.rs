// Generated macro for sign (function)
macro_rules! Depcrate_repository_commitsign {
() => {
// Module: crate::repository::commit
// Provides: {"sign"}
// Dependencies: {}
# [doc = " Note that this is a quick first prototype that lacks some of the features provided by `git verify-commit`."] pub fn sign (repo : gix :: Repository , rev_spec : Option < & str > , mut out : impl std :: io :: Write) -> Result < () > { let rev_spec = rev_spec . unwrap_or ("HEAD") ; let object = repo . rev_parse_single (format ! ("{rev_spec}^{{commit}}") . as_str ()) ? . object () ? ; let mut commit_ref = object . to_commit_ref () ; if commit_ref . extra_headers () . pgp_signature () . is_some () { gix :: trace :: info ! ("The commit {id} is already signed, did nothing" , id = object . id) ; writeln ! (out , "{id}" , id = object . id) ? ; return Ok (()) ; } let mut cmd : std :: process :: Command = gix :: command :: prepare ("gpg") . into () ; cmd . args (["--keyid-format=long" , "--status-fd=2" , "--detach-sign" , "--sign" , "--armor" ,]) . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) ; gix :: trace :: debug ! ("About to execute {cmd:?}") ; let mut child = cmd . spawn () ? ; child . stdin . take () . expect ("to be present") . write_all (& object . data) ? ; if ! child . wait () ? . success () { bail ! ("Command {cmd:?} failed") ; } let mut signed_data = Vec :: new () ; child . stdout . expect ("to be present") . read_to_end (& mut signed_data) ? ; commit_ref . extra_headers . push ((BStr :: new (SIGNATURE_FIELD_NAME) , Cow :: Owned (BString :: new (signed_data)))) ; let signed_id = repo . write_object (& commit_ref) ? ; writeln ! (& mut out , "{signed_id}") ? ; Ok (()) }
};
}
