// Generated macro for run_bench (function)
macro_rules! Depcraterun_bench {
() => {
// Module: crate
// Provides: {"run_bench"}
// Dependencies: {}
fn run_bench (arch : & str , executable : & str , i : isize , name : & str , allow_aslr : bool ,) -> (CachegrindStats , Option < CachegrindStats >) { let output_file = PathBuf :: from (format ! ("target/iai/cachegrind.out.{}" , name)) ; let old_file = output_file . with_file_name (format ! ("cachegrind.out.{}.old" , name)) ; std :: fs :: create_dir_all (output_file . parent () . unwrap ()) . expect ("Failed to create directory") ; if output_file . exists () { std :: fs :: copy (& output_file , & old_file) . unwrap () ; } let mut cmd = if allow_aslr { basic_valgrind () } else { valgrind_without_aslr (arch) } ; let status = cmd . arg ("--tool=cachegrind") . arg ("--I1=32768,8,64") . arg ("--D1=32768,8,64") . arg ("--LL=8388608,16,64") . arg (format ! ("--cachegrind-out-file={}" , output_file . display ())) . arg (executable) . arg ("--iai-run") . arg (i . to_string ()) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . status () . expect ("Failed to run benchmark in cachegrind") ; if ! status . success () { panic ! ("Failed to run benchmark in cachegrind. Exit code: {}" , status) ; } let new_stats = parse_cachegrind_output (& output_file) ; let old_stats = if old_file . exists () { Some (parse_cachegrind_output (& old_file)) } else { None } ; (new_stats , old_stats) }
};
}
