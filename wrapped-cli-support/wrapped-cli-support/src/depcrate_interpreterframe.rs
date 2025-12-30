// Generated macro for Frame (struct)
macro_rules! Depcrate_interpreterFrame {
() => {
// Module: crate::interpreter
// Provides: {"Frame"}
// Dependencies: {}
struct Frame < 'a > { module : & 'a Module , func : & 'a LocalFunction , interp : & 'a mut Interpreter , locals : BTreeMap < LocalId , i32 > , }
};
}
