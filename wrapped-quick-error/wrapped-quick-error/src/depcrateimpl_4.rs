// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl < T , E > ResultExt < T , E > for Result < T , E > { fn context < X > (self , x : X) -> Result < T , Context < X , E > > { self . map_err (| e | Context (x , e)) } }
};
}
