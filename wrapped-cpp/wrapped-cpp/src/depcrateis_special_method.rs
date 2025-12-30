// Generated macro for is_special_method (function)
macro_rules! Depcrateis_special_method {
() => {
// Module: crate
// Provides: {"is_special_method"}
// Dependencies: {}
fn is_special_method (func : & Function) -> SpecialMethod { if matches ! (func . kind , FunctionKind :: Static (_)) { if func . name . starts_with ("[resource-drop]") { SpecialMethod :: ResourceDrop } else if func . name . starts_with ("[resource-new]") { SpecialMethod :: ResourceNew } else if func . name . starts_with ("[resource-rep]") { SpecialMethod :: ResourceRep } else if func . name . starts_with ("[dtor]") { SpecialMethod :: Dtor } else if func . name == "$alloc" { SpecialMethod :: Allocate } else { SpecialMethod :: None } } else { SpecialMethod :: None } }
};
}
