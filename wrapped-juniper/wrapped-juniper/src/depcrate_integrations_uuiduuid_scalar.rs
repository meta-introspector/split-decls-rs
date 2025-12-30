// Generated macro for uuid_scalar (module)
macro_rules! Depcrate_integrations_uuiduuid_scalar {
() => {
// Module: crate::integrations::uuid
// Provides: {"uuid_scalar"}
// Dependencies: {}
mod uuid_scalar { use super :: Uuid ; pub (super) fn from_input (s : & str) -> Result < Uuid , Box < str > > { Uuid :: parse_str (s) . map_err (| e | format ! ("Failed to parse `UUID`: {e}") . into ()) } }
};
}
