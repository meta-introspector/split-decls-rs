// Generated macro for impl_1107 (impl)
macro_rules! Depcrate_read_wasmimpl_1107 {
() => {
// Module: crate::read::wasm
// Provides: {"impl_1107"}
// Dependencies: {}
impl < 'data , 'file > ObjectSymbol < 'data > for WasmSymbol < 'data , 'file > { # [inline] fn index (& self) -> SymbolIndex { self . index } # [inline] fn name_bytes (& self) -> read :: Result < & 'data [u8] > { Ok (self . symbol . name . as_bytes ()) } # [inline] fn name (& self) -> read :: Result < & 'data str > { Ok (self . symbol . name) } # [inline] fn address (& self) -> u64 { self . symbol . address } # [inline] fn size (& self) -> u64 { self . symbol . size } # [inline] fn kind (& self) -> SymbolKind { self . symbol . kind } # [inline] fn section (& self) -> SymbolSection { self . symbol . section } # [inline] fn is_undefined (& self) -> bool { self . symbol . section == SymbolSection :: Undefined } # [inline] fn is_definition (& self) -> bool { (self . symbol . kind == SymbolKind :: Text || self . symbol . kind == SymbolKind :: Data) && self . symbol . section != SymbolSection :: Undefined } # [inline] fn is_common (& self) -> bool { self . symbol . section == SymbolSection :: Common } # [inline] fn is_weak (& self) -> bool { self . symbol . weak } # [inline] fn scope (& self) -> SymbolScope { self . symbol . scope } # [inline] fn is_global (& self) -> bool { self . symbol . scope != SymbolScope :: Compilation } # [inline] fn is_local (& self) -> bool { self . symbol . scope == SymbolScope :: Compilation } # [inline] fn flags (& self) -> SymbolFlags < SectionIndex , SymbolIndex > { SymbolFlags :: None } }
};
}
