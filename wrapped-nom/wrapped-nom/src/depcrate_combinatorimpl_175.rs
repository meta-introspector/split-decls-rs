// Generated macro for impl_175 (impl)
macro_rules! Depcrate_combinatorimpl_175 {
() => {
// Module: crate::combinator
// Provides: {"impl_175"}
// Dependencies: {}
impl < I , O , E > Parser < I > for Success < O , E > where O : Clone , E : ParseError < I > , { type Output = O ; type Error = E ; fn process < OM : OutputMode > (& mut self , input : I) -> PResult < OM , I , Self :: Output , Self :: Error > { Ok ((input , OM :: Output :: bind (| | self . val . clone ()))) } }
};
}
