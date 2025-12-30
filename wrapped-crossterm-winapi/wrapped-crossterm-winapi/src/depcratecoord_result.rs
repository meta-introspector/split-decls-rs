// Generated macro for coord_result (function)
macro_rules! Depcratecoord_result {
() => {
// Module: crate
// Provides: {"coord_result"}
// Dependencies: {}
# [doc = " Get the result of a call to WinAPI that returns a"] # [doc = " [`COORD`](https://docs.microsoft.com/en-us/windows/console/coord-str) as an [`io::Result`]."] # [inline] pub fn coord_result (return_value : COORD) -> io :: Result < Coord > { if return_value . X != 0 && return_value . Y != 0 { Ok (Coord :: from (return_value)) } else { Err (io :: Error :: last_os_error ()) } }
};
}
