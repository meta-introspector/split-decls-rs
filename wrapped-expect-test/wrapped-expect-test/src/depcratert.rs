// Generated macro for RT (static)
macro_rules! DepcrateRT {
() => {
// Module: crate
// Provides: {"RT"}
// Dependencies: {}
static RT : Lazy < Mutex < Runtime > > = Lazy :: new (Default :: default) ;
};
}
