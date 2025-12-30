// Generated macro for flags (macro)
macro_rules! Depcrate_readobjflags {
() => {
// Module: crate::readobj
// Provides: {"flags"}
// Dependencies: {}
macro_rules ! flags { ($ ($ name : ident) ,+ $ (,) ?) => ([$ (Flag { value : $ name , name : stringify ! ($ name) , }) ,+]) }
};
}
