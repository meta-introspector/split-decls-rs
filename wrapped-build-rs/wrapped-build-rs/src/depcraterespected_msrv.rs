// Generated macro for respected_msrv (macro)
macro_rules! Depcraterespected_msrv {
() => {
// Module: crate
// Provides: {"respected_msrv"}
// Dependencies: {}
macro_rules ! respected_msrv { ($ ver : literal) => { concat ! (r#"<div class="warning">

MSRV: Respected as of "# , $ ver , r#".

</div>"#) } ; }
};
}
