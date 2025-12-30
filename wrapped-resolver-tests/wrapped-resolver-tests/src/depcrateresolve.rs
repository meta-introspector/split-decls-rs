// Generated macro for resolve (function)
macro_rules! Depcrateresolve {
() => {
// Module: crate
// Provides: {"resolve"}
// Dependencies: {}
pub fn resolve (deps : Vec < Dependency > , registry : & [Summary]) -> CargoResult < Vec < PackageId > > { Ok (resolve_with_global_context (deps , registry , & GlobalContext :: default () . unwrap ()) ? . into_iter () . map (| (pkg , _) | pkg) . collect () ,) }
};
}
