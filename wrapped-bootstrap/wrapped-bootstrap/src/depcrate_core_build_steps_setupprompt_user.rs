// Generated macro for prompt_user (function)
macro_rules! Depcrate_core_build_steps_setupprompt_user {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"prompt_user"}
// Dependencies: {}
# [doc = " Prompt a user for a answer, looping until they enter an accepted input or nothing"] fn prompt_user (prompt : & str) -> io :: Result < Option < PromptResult > > { let mut input = String :: new () ; loop { print ! ("{prompt} ") ; io :: stdout () . flush () ? ; input . clear () ; io :: stdin () . read_line (& mut input) ? ; match input . trim () . to_lowercase () . as_str () { "y" | "yes" => return Ok (Some (PromptResult :: Yes)) , "n" | "no" => return Ok (Some (PromptResult :: No)) , "p" | "print" => return Ok (Some (PromptResult :: Print)) , "" => return Ok (None) , _ => { eprintln ! ("ERROR: unrecognized option '{}'" , input . trim ()) ; eprintln ! ("NOTE: press Ctrl+C to exit") ; } } ; } }
};
}
