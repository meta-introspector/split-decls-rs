// Generated macro for _generate (function)
macro_rules! Depcrate_aot_generator_generate {
() => {
// Module: crate::aot::generator
// Provides: {"_generate"}
// Dependencies: {}
fn _generate < G : Generator > (generator : G , cmd : & mut Command , buf : & mut dyn Write) { cmd . build () ; generator . generate (cmd , buf) ; }
};
}
