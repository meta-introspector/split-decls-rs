// Generated macro for unix (module)
macro_rules! Depcrateunix {
() => {
// Module: crate
// Provides: {"unix"}
// Dependencies: {}
# [cfg (unix)] mod unix { use std :: os :: unix :: fs :: PermissionsExt ; use std :: path :: Path ; use super :: IsExecutable ; impl IsExecutable for Path { fn is_executable (& self) -> bool { let metadata = match self . metadata () { Ok (metadata) => metadata , Err (_) => return false , } ; let permissions = metadata . permissions () ; metadata . is_file () && permissions . mode () & 0o111 != 0 } } }
};
}
