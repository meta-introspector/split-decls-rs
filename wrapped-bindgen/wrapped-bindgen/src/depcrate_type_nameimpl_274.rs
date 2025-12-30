// Generated macro for impl_274 (impl)
macro_rules! Depcrate_type_nameimpl_274 {
() => {
// Module: crate::type_name
// Provides: {"impl_274"}
// Dependencies: {}
impl TypeName { pub const Object : Self = Self ("System" , "Object") ; pub const IsConst : Self = Self ("System.Runtime.CompilerServices" , "IsConst") ; pub const IAsyncAction : Self = Self ("Windows.Foundation" , "IAsyncAction") ; pub const IAsyncActionWithProgress : Self = Self ("Windows.Foundation" , "IAsyncActionWithProgress") ; pub const IAsyncOperation : Self = Self ("Windows.Foundation" , "IAsyncOperation") ; pub const IAsyncOperationWithProgress : Self = Self ("Windows.Foundation" , "IAsyncOperationWithProgress") ; pub const IIterable : Self = Self ("Windows.Foundation.Collections" , "IIterable") ; pub const IIterator : Self = Self ("Windows.Foundation.Collections" , "IIterator") ; pub const VARIANT : Self = Self ("Windows.Win32.System.Variant" , "VARIANT") ; pub const PROPVARIANT : Self = Self ("Windows.Win32.System.Com.StructuredStorage" , "PROPVARIANT") ; pub fn parse (full_name : & 'static str) -> Self { let index = full_name . rfind ('.') . expect ("Expected full name separated with `.`") ; Self (& full_name [0 .. index] , & full_name [index + 1 ..]) } pub fn namespace (& self) -> & 'static str { self . 0 } pub fn name (& self) -> & 'static str { self . 1 } pub fn write (& self , config : & Config , generics : & [Type]) -> TokenStream { let name = to_ident (self . name ()) ; let namespace = config . write_namespace (* self) ; if generics . is_empty () { quote ! { # namespace # name } } else { let generics = generics . iter () . map (| ty | ty . write_name (config)) ; quote ! { # namespace # name < # (# generics) ,* > } } } }
};
}
