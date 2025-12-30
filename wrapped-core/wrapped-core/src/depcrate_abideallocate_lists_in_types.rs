// Generated macro for deallocate_lists_in_types (function)
macro_rules! Depcrate_abideallocate_lists_in_types {
() => {
// Module: crate::abi
// Provides: {"deallocate_lists_in_types"}
// Dependencies: {}
# [doc = " Generate instructions in `bindgen` to deallocate all lists in `ptr` where"] # [doc = " that's a pointer to a sequence of `types` stored in linear memory."] pub fn deallocate_lists_in_types < B : Bindgen > (resolve : & Resolve , types : & [Type] , operands : & [B :: Operand] , indirect : bool , bindgen : & mut B ,) { Generator :: new (resolve , bindgen) . deallocate_in_types (types , operands , indirect , Deallocate :: Lists ,) ; }
};
}
