// Generated macro for impl_16 (impl)
macro_rules! Depcrate_loggingimpl_16 {
() => {
// Module: crate::logging
// Provides: {"impl_16"}
// Dependencies: {}
# [cfg (all (test , feature = "std" , feature = "logging"))] impl log :: Log for Logger { fn enabled (& self , _ : & log :: Metadata < '_ >) -> bool { true } fn log (& self , record : & log :: Record < '_ >) { match (record . file () , record . line ()) { (Some (file) , Some (line)) => { std :: eprintln ! ("{}|{}|{}:{}: {}" , record . level () , record . target () , file , line , record . args ()) ; } (Some (file) , None) => { std :: eprintln ! ("{}|{}|{}: {}" , record . level () , record . target () , file , record . args ()) ; } _ => { std :: eprintln ! ("{}|{}: {}" , record . level () , record . target () , record . args ()) ; } } } fn flush (& self) { } }
};
}
