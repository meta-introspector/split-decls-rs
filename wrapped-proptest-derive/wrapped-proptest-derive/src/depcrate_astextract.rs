// Generated macro for extract (function)
macro_rules! Depcrate_astextract {
() => {
// Module: crate::ast
// Provides: {"extract"}
// Dependencies: {}
# [doc = " Wraps a `Ctor` that expects the `to` \"register\" to be filled with"] # [doc = " contents of the `from` register. The correctness of this wrt. the"] # [doc = " generated Rust code has to be verified externally by checking the"] # [doc = " construction of the particular `Ctor`."] fn extract (c : Ctor , to : ToReg , from : FromReg) -> Ctor { Ctor :: Extract (Box :: new (c) , to , from) }
};
}
