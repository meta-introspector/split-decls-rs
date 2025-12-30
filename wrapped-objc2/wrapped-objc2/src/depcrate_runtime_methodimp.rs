// Generated macro for Imp (type)
macro_rules! Depcrate_runtime_methodImp {
() => {
// Module: crate::runtime::method
// Provides: {"Imp"}
// Dependencies: {}
# [doc = " A pointer to the start of a method implementation."] # [doc = ""] # [doc = " The first argument is a pointer to the receiver, the second argument is"] # [doc = " the selector, and the rest of the arguments follow."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This is a \"catch all\" type; it must be transmuted to the correct type"] # [doc = " before being called!"] # [doc = ""] # [doc = " Also note that this is non-null! If you require an Imp that can be null,"] # [doc = " use `Option<Imp>`."] # [doc (alias = "IMP")] pub type Imp = unsafe extern "C-unwind" fn () ;
};
}
