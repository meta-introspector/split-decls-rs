// Generated macro for impl_106 (impl)
macro_rules! Depcrate_hours_utilimpl_106 {
() => {
// Module: crate::hours::util
// Provides: {"impl_106"}
// Dependencies: {}
impl WorkByPerson { pub fn write_to (& self , total_hours : f32 , total_files : Option < FileStats > , total_lines : Option < LineStats > , mut out : impl std :: io :: Write ,) -> std :: io :: Result < () > { writeln ! (out , "{names} <{mails}>" , names = join (self . name . iter () , ", ") , mails = join (self . email . iter () , ", ")) ? ; writeln ! (out , "{} commits found" , self . num_commits) ? ; writeln ! (out , "total time spent: {:.02}h ({:.02} 8h days, {:.02}%)" , self . hours , self . hours / HOURS_PER_WORKDAY , (self . hours / total_hours) * 100.0) ? ; if let Some (total) = total_files { writeln ! (out , "total files added/removed/modified: {}/{}/{} ({:.02}%)" , self . files . added , self . files . removed , self . files . modified , (self . files . sum () / total . sum ()) * 100.0) ? ; } if let Some (total) = total_lines { writeln ! (out , "total lines added/removed: {}/{} ({:.02}%)" , self . lines . added , self . lines . removed , (self . lines . sum () / total . sum ()) * 100.0) ? ; } Ok (()) } }
};
}
