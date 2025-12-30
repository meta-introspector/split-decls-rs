// Generated macro for requires_msrv (macro)
macro_rules! Depcraterequires_msrv {
() => {
// Module: crate
// Provides: {"requires_msrv"}
// Dependencies: {}
macro_rules ! requires_msrv { ($ ver : literal) => { concat ! (r#"<div class="warning">

MSRV: Requires "# , $ ver , r#".

</div>"#) } ; }
};
}
