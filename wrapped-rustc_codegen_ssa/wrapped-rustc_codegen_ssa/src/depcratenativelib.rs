// Generated macro for NativeLib (struct)
macro_rules! DepcrateNativeLib {
() => {
// Module: crate
// Provides: {"NativeLib"}
// Dependencies: {}
# [derive (Clone , Debug , Encodable , Decodable , HashStable)] pub struct NativeLib { pub kind : NativeLibKind , pub name : Symbol , pub filename : Option < Symbol > , pub cfg : Option < CfgEntry > , pub verbatim : bool , pub dll_imports : Vec < cstore :: DllImport > , }
};
}
