// Generated macro for parse_line (function)
macro_rules! Depcrate_parseparse_line {
() => {
// Module: crate::parse
// Provides: {"parse_line"}
// Dependencies: {}
fn parse_line (line : & BStr , line_number : usize) -> Result < Entry < '_ > , Error > { let (name1 , email1 , rest) = parse_name_and_email (line , line_number) ? ; let (name2 , email2 , rest) = parse_name_and_email (rest , line_number) ? ; if ! rest . trim () . is_empty () { return Err (Error :: UnconsumedInput { line_number , line : line . into () , }) ; } Ok (match (name1 , email1 , name2 , email2) { (Some (proper_name) , Some (commit_email) , None , None) => Entry :: change_name_by_email (proper_name , commit_email) , (None , Some (proper_email) , None , Some (commit_email)) => { Entry :: change_email_by_email (proper_email , commit_email) } (Some (proper_name) , Some (proper_email) , None , Some (commit_email)) => { Entry :: change_name_and_email_by_email (proper_name , proper_email , commit_email) } (Some (proper_name) , Some (proper_email) , Some (commit_name) , Some (commit_email)) => { Entry :: change_name_and_email_by_name_and_email (proper_name , proper_email , commit_name , commit_email) } (None , Some (proper_email) , Some (commit_name) , Some (commit_email)) => { Entry :: change_email_by_name_and_email (proper_email , commit_name , commit_email) } _ => { return Err (Error :: Malformed { line_number , line : line . into () , message : "Emails without a name or email to map to are invalid" . into () , }) } }) }
};
}
