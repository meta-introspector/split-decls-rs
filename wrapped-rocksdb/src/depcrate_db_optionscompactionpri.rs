// Generated macro for CompactionPri (enum)
macro_rules! Depcrate_db_optionsCompactionPri {
() => {
// Module: crate::db_options
// Provides: {"CompactionPri"}
// Dependencies: {}
# [repr (i32)] pub enum CompactionPri { # [doc = " Slightly prioritize larger files by size compensated by #deletes"] ByCompensatedSize = 0 , # [doc = " First compact files whose data's latest update time is oldest."] # [doc = " Try this if you only update some hot keys in small ranges."] OldestLargestSeqFirst = 1 , # [doc = " First compact files whose range hasn't been compacted to the next level"] # [doc = " for the longest. If your updates are random across the key space,"] # [doc = " write amplification is slightly better with this option."] OldestSmallestSeqFirst = 2 , # [doc = " First compact files whose ratio between overlapping size in next level"] # [doc = " and its size is the smallest. It in many cases can optimize write amplification."] MinOverlappingRatio = 3 , # [doc = " Keeps a cursor(s) of the successor of the file (key range) was/were"] # [doc = " compacted before, and always picks the next files (key range) in that"] # [doc = " level. The file picking process will cycle through all the files in a"] # [doc = " round-robin manner."] RoundRobin = 4 , }
};
}
