// Generated macro for read2_abbreviated (function)
macro_rules! Depcrate_read2read2_abbreviated {
() => {
// Module: crate::read2
// Provides: {"read2_abbreviated"}
// Dependencies: {}
pub fn read2_abbreviated (mut child : Child , filter_paths_from_len : & [String] ,) -> io :: Result < (Output , Truncated) > { let mut stdout = ProcOutput :: new () ; let mut stderr = ProcOutput :: new () ; drop (child . stdin . take ()) ; read2 (child . stdout . take () . unwrap () , child . stderr . take () . unwrap () , & mut | is_stdout , data , _ | { if is_stdout { & mut stdout } else { & mut stderr } . extend (data , filter_paths_from_len) ; data . clear () ; } ,) ? ; let status = child . wait () ? ; let truncated = if stdout . truncated () || stderr . truncated () { Truncated :: Yes } else { Truncated :: No } ; Ok ((Output { status , stdout : stdout . into_bytes () , stderr : stderr . into_bytes () } , truncated)) }
};
}
