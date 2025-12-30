// Generated macro for async_recursive (macro)
macro_rules! Depcrateasync_recursive {
() => {
// Module: crate
// Provides: {"async_recursive"}
// Dependencies: {}
macro_rules ! async_recursive { (13 , $ inner : expr) => { async { async_recursive ! (12 , $ inner) } . await } ; (12 , $ inner : expr) => { async { async_recursive ! (11 , $ inner) } . await } ; (11 , $ inner : expr) => { async { async_recursive ! (10 , $ inner) } . await } ; (10 , $ inner : expr) => { async { async_recursive ! (9 , $ inner) } . await } ; (9 , $ inner : expr) => { async { async_recursive ! (8 , $ inner) } . await } ; (8 , $ inner : expr) => { async { async_recursive ! (7 , $ inner) } . await } ; (7 , $ inner : expr) => { async { async_recursive ! (6 , $ inner) } . await } ; (6 , $ inner : expr) => { async { async_recursive ! (5 , $ inner) } . await } ; (5 , $ inner : expr) => { async { async_recursive ! (4 , $ inner) } . await } ; (4 , $ inner : expr) => { async { async_recursive ! (3 , $ inner) } . await } ; (3 , $ inner : expr) => { async { async_recursive ! (2 , $ inner) } . await } ; (2 , $ inner : expr) => { async { async_recursive ! (1 , $ inner) } . await } ; (1 , $ inner : expr) => { async { async_recursive ! (0 , $ inner) } . await } ; (0 , $ inner : expr) => { async { h19 (& String :: from ("owo") , & 0) . await ; $ inner } . await } ; }
};
}
