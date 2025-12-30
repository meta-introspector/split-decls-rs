// Generated macro for create_test_files (function)
macro_rules! Depcrate_tests_sqpollcreate_test_files {
() => {
// Module: crate::tests::sqpoll
// Provides: {"create_test_files"}
// Dependencies: {}
# [doc = " Create test files for I/O operations"] fn create_test_files (dir : & Path , count : usize) -> Vec < File > { let mut files = Vec :: new () ; for i in 0 .. count { let file_path = dir . join (format ! ("test_file_{}.txt" , i)) ; let mut file = File :: create (& file_path) . expect ("Failed to create test file") ; let content = format ! ("Test content for file {}" , i) ; file . write_all (content . as_bytes ()) . expect ("Failed to write to test file") ; file . flush () . expect ("Failed to flush file") ; let read_file = File :: open (& file_path) . expect ("Failed to open test file for reading") ; files . push (read_file) ; } files }
};
}
