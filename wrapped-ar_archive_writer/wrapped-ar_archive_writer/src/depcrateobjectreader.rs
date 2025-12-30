// Generated macro for ObjectReader (struct)
macro_rules! DepcrateObjectReader {
() => {
// Module: crate
// Provides: {"ObjectReader"}
// Dependencies: {}
# [doc = " Helper struct to query object file information from members."] pub struct ObjectReader { # [doc = " Iterates over the symbols in the object file."] pub get_symbols : GetSymbolsFn , # [doc = " Returns true if the object file is 64-bit."] # [doc = " Note that this should match LLVM's `SymbolicFile::is64Bit`, which"] # [doc = " considers all COFF files to be 32-bit."] pub is_64_bit_object_file : Is64BitObjectFileFn , # [doc = " Returns true if the object file is an EC (that is, an Arm64EC or x64)"] # [doc = " object file"] pub is_ec_object_file : IsECObjectFileFn , # [doc = " Returns true if the object file is any Arm64 (Native Arm64, Arm64EC or"] # [doc = " Arm64X) COFF file."] pub is_any_arm64_coff : IsAnyArm64CoffFn , # [doc = " Returns the member alignment of an XCoff object file."] pub get_xcoff_member_alignment : GetXCoffMemberAlignmentFn , }
};
}
