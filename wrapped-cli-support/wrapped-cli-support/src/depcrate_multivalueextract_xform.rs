// Generated macro for extract_xform (function)
macro_rules! Depcrate_multivalueextract_xform {
() => {
// Module: crate::multivalue
// Provides: {"extract_xform"}
// Dependencies: {}
fn extract_xform (module : & Module , adapter : & mut Adapter , to_xform : & mut Vec < (walrus :: FunctionId , usize , Vec < walrus :: ValType >) > , exports : & mut Vec < walrus :: ExportId > ,) { let instructions = match & mut adapter . kind { AdapterKind :: Local { instructions } => instructions , AdapterKind :: Import { .. } => return , } ; if let Some (Instruction :: Retptr { .. }) = instructions . first () . map (| e | & e . instr) { instructions . remove (0) ; let mut types = Vec :: new () ; instructions . retain (| instruction | match & instruction . instr { Instruction :: LoadRetptr { ty , .. } => { types . push (ty . to_wasm () . unwrap ()) ; false } _ => true , }) ; let export = instructions . iter_mut () . find_map (| i | match i . instr { Instruction :: CallExport (e) => Some (e) , _ => None , }) . expect ("adapter never calls the underlying function") ; let id = match module . exports . get (export) . item { walrus :: ExportItem :: Function (f) => f , _ => panic ! ("found call to non-function export") , } ; to_xform . push ((id , 0 , types)) ; exports . push (export) ; } }
};
}
