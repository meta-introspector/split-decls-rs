// Generated macro for parse_headers_iter (function)
macro_rules! Depcrateparse_headers_iter {
() => {
// Module: crate
// Provides: {"parse_headers_iter"}
// Dependencies: {}
# [inline] fn parse_headers_iter < 'a > (headers : & mut & mut [Header < 'a >] , bytes : & mut Bytes < 'a > , config : & HeaderParserConfig ,) -> Result < usize > { parse_headers_iter_uninit (unsafe { deinit_slice_mut (headers) } , bytes , config ,) }
};
}
