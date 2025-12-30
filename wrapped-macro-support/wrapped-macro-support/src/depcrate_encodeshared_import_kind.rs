// Generated macro for shared_import_kind (function)
macro_rules! Depcrate_encodeshared_import_kind {
() => {
// Module: crate::encode
// Provides: {"shared_import_kind"}
// Dependencies: {}
fn shared_import_kind < 'a > (i : & 'a ast :: ImportKind , intern : & 'a Interner ,) -> Result < ImportKind < 'a > , Diagnostic > { Ok (match i { ast :: ImportKind :: Function (f) => ImportKind :: Function (shared_import_function (f , intern) ?) , ast :: ImportKind :: Static (f) => ImportKind :: Static (shared_import_static (f , intern)) , ast :: ImportKind :: String (f) => ImportKind :: String (shared_import_string (f , intern)) , ast :: ImportKind :: Type (f) => ImportKind :: Type (shared_import_type (f , intern)) , ast :: ImportKind :: Enum (f) => ImportKind :: Enum (shared_import_enum (f , intern)) , }) }
};
}
