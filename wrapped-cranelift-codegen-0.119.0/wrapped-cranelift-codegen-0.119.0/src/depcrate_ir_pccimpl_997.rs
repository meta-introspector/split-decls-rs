// Generated macro for impl_997 (impl)
macro_rules! Depcrate_ir_pccimpl_997 {
() => {
// Module: crate::ir::pcc
// Provides: {"impl_997"}
// Dependencies: {}
impl fmt :: Display for Fact { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Fact :: Range { bit_width , min , max , } => write ! (f , "range({bit_width}, {min:#x}, {max:#x})") , Fact :: DynamicRange { bit_width , min , max , } => { write ! (f , "dynamic_range({bit_width}, {min}, {max})") } Fact :: Mem { ty , min_offset , max_offset , nullable , } => { let nullable_flag = if * nullable { ", nullable" } else { "" } ; write ! (f , "mem({ty}, {min_offset:#x}, {max_offset:#x}{nullable_flag})") } Fact :: DynamicMem { ty , min , max , nullable , } => { let nullable_flag = if * nullable { ", nullable" } else { "" } ; write ! (f , "dynamic_mem({ty}, {min}, {max}{nullable_flag})") } Fact :: Def { value } => write ! (f , "def({value})") , Fact :: Compare { kind , lhs , rhs } => { write ! (f , "compare({kind}, {lhs}, {rhs})") } Fact :: Conflict => write ! (f , "conflict") , } } }
};
}
