// Generated macro for impl_285 (impl)
macro_rules! Depcrate_runtimeimpl_285 {
() => {
// Module: crate::runtime
// Provides: {"impl_285"}
// Dependencies: {}
impl < 'a > From < BinarySnapshotValue < 'a > > for SnapshotValue < 'a > { fn from (BinarySnapshotValue { name_and_extension , content , } : BinarySnapshotValue < 'a > ,) -> Self { let (name , extension) = name_and_extension . split_once ('.') . unwrap_or_else (| | { panic ! ("\"{name_and_extension}\" does not match the format \"name.extension\"" ,) }) ; let name = if name . is_empty () { None } else { Some (Cow :: Borrowed (name)) } ; SnapshotValue :: Binary { name , extension , content , } } }
};
}
