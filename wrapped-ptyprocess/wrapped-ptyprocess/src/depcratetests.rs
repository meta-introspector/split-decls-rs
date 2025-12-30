// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn create_pty () -> Result < () > { let master = Master :: open () ? ; master . grant_slave_access () ? ; master . unlock_slave () ? ; let slavename = master . get_slave_name () ? ; let expected_path = if cfg ! (target_os = "freebsd") { "pts/" } else if cfg ! (target_os = "macos") { "/dev/ttys" } else { "/dev/pts/" } ; if ! slavename . starts_with (expected_path) { assert_eq ! (expected_path , slavename) ; } Ok (()) } # [test] # [ignore = "The test should be run in a sigle thread mode --jobs 1 or --test-threads 1"] fn release_pty_master () -> Result < () > { let master = Master :: open () ? ; let old_master_fd = master . fd . as_raw_fd () ; drop (master) ; let master = Master :: open () ? ; assert ! (master . fd . as_raw_fd () == old_master_fd) ; Ok (()) } # [cfg (feature = "close-range")] # [test] fn test_get_ranges () { assert_eq ! (get_untouched_fds (& []) , vec ! [0 .. RawFd :: MAX]) ; assert_eq ! (get_untouched_fds (& [RawFd :: MAX]) , vec ! [0 .. RawFd :: MAX]) ; assert_eq ! (get_untouched_fds (& [10 , RawFd :: MAX]) , vec ! [0 .. 10 , 11 .. RawFd :: MAX]) ; assert_eq ! (get_untouched_fds (& [100]) , vec ! [0 .. 100 , 101 .. RawFd :: MAX]) ; assert_eq ! (get_untouched_fds (& [10 , 20]) , vec ! [0 .. 10 , 11 .. 20 , 21 .. RawFd :: MAX]) ; assert_eq ! (get_untouched_fds (& [1 , 2 , 10 , 20]) , vec ! [0 .. 1 , 3 .. 10 , 11 .. 20 , 21 .. RawFd :: MAX]) ; assert_eq ! (get_untouched_fds (& [0 , 1 , 2 , 10 , 20]) , vec ! [3 .. 10 , 11 .. 20 , 21 .. RawFd :: MAX]) ; } }
};
}
