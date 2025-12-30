// Generated macro for InterfaceGenerator (struct)
macro_rules! Depcrate_interfaceInterfaceGenerator {
() => {
// Module: crate::interface
// Provides: {"InterfaceGenerator"}
// Dependencies: {}
# [doc = " InterfaceGenerator generates the C# code for wit interfaces."] # [doc = " It produces types by interface in wit and then generates the interop code"] # [doc = " by calling out to FunctionGenerator"] pub (crate) struct InterfaceGenerator < 'a > { pub (crate) src : String , pub (crate) csharp_interop_src : String , pub (crate) stub : String , pub (crate) csharp_gen : & 'a mut CSharp , pub (crate) resolve : & 'a Resolve , pub (crate) name : & 'a str , pub (crate) direction : Direction , }
};
}
