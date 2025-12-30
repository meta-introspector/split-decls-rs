// Generated macro for execute (function)
macro_rules! Depcrate_descriptorsexecute {
() => {
// Module: crate::descriptors
// Provides: {"execute"}
// Dependencies: {}
# [doc = " Execute all `__wbindgen_describe_*` functions in a module, inserting a"] # [doc = " custom section which represents the executed value of each descriptor."] # [doc = ""] # [doc = " Afterwards this will delete all descriptor functions from the module."] pub fn execute (module : & mut Module) -> Result < WasmBindgenDescriptorsSectionId , Error > { let mut section = WasmBindgenDescriptorsSection :: default () ; let mut interpreter = Interpreter :: new (module) ? ; section . execute_exports (module , & mut interpreter) ? ; section . execute_casts (module , & mut interpreter) ? ; Ok (module . customs . add (section)) }
};
}
