// Generated macro for impl_210 (impl)
macro_rules! Depcrate_transforms_multi_value_testsimpl_210 {
() => {
// Module: crate::transforms::multi_value::tests
// Provides: {"impl_210"}
// Dependencies: {}
impl < 'a > Parse < 'a > for Directive { fn parse (parser : Parser < 'a >) -> wast :: parser :: Result < Self > { use wast :: { core :: ValType , kw } ; parser . parse :: < kw :: export > () ? ; let name = parser . parse () ? ; let mut tys = Vec :: new () ; parser . parens (| p | { while ! p . is_empty () { tys . push (match p . parse () ? { ValType :: I32 => walrus :: ValType :: I32 , ValType :: I64 => walrus :: ValType :: I64 , ValType :: F32 => walrus :: ValType :: F32 , ValType :: F64 => walrus :: ValType :: F64 , _ => panic ! () , }) ; } Ok (()) }) ? ; Ok (Directive { name , tys }) } }
};
}
