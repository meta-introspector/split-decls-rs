// Generated macro for DEFAULT_OBJECT_READER (const)
macro_rules! DepcrateDEFAULT_OBJECT_READER {
() => {
// Module: crate
// Provides: {"DEFAULT_OBJECT_READER"}
// Dependencies: {}
# [doc = " Default implementation of [ObjectReader] that uses the `object` crate."] pub const DEFAULT_OBJECT_READER : ObjectReader = ObjectReader { get_symbols : object_reader :: get_native_object_symbols , is_64_bit_object_file : object_reader :: is_64_bit_symbolic_file , is_ec_object_file : object_reader :: is_ec_object , is_any_arm64_coff : object_reader :: is_any_arm64_coff , get_xcoff_member_alignment : object_reader :: get_member_alignment , } ;
};
}
