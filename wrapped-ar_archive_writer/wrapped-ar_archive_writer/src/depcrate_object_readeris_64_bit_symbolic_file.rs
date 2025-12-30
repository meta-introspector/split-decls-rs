// Generated macro for is_64_bit_symbolic_file (function)
macro_rules! Depcrate_object_readeris_64_bit_symbolic_file {
() => {
// Module: crate::object_reader
// Provides: {"is_64_bit_symbolic_file"}
// Dependencies: {}
pub fn is_64_bit_symbolic_file (obj : & [u8]) -> bool { object :: FileKind :: parse (obj) . is_ok_and (| kind | match kind { object :: FileKind :: Elf64 | object :: FileKind :: MachO64 | object :: FileKind :: Pe64 | object :: FileKind :: Xcoff64 | object :: FileKind :: MachOFat64 => true , object :: FileKind :: Elf32 | object :: FileKind :: MachO32 | object :: FileKind :: Pe32 | object :: FileKind :: Xcoff32 | object :: FileKind :: MachOFat32 | object :: FileKind :: Coff | object :: FileKind :: CoffBig | object :: FileKind :: CoffImport => false , _ => panic ! ("Unexpected file kind") , }) }
};
}
