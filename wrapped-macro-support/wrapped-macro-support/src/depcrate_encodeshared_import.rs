// Generated macro for shared_import (function)
macro_rules! Depcrate_encodeshared_import {
() => {
// Module: crate::encode
// Provides: {"shared_import"}
// Dependencies: {}
fn shared_import < 'a > (i : & 'a ast :: Import , intern : & 'a Interner) -> Result < Import < 'a > , Diagnostic > { Ok (Import { module : i . module . as_ref () . map (| m | shared_module (m , intern , false)) . transpose () ? , js_namespace : i . js_namespace . clone () , reexport : i . reexport . clone () , kind : shared_import_kind (& i . kind , intern) ? , }) }
};
}
