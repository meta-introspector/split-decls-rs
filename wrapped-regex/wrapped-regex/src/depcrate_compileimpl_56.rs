// Generated macro for impl_56 (impl)
macro_rules! Depcrate_compileimpl_56 {
() => {
// Module: crate::compile
// Provides: {"impl_56"}
// Dependencies: {}
impl MaybeInst { fn fill (& mut self , goto : InstPtr) { let filled = match * self { MaybeInst :: Uncompiled (ref inst) => inst . fill (goto) , MaybeInst :: Split1 (goto1) => { Inst :: Split (InstSplit { goto1 : goto1 , goto2 : goto }) } MaybeInst :: Split2 (goto2) => { Inst :: Split (InstSplit { goto1 : goto , goto2 : goto2 }) } _ => unreachable ! ("not all instructions were compiled! \
                               found uncompiled instruction: {:?}" , self) , } ; * self = MaybeInst :: Compiled (filled) ; } fn fill_split (& mut self , goto1 : InstPtr , goto2 : InstPtr) { let filled = match * self { MaybeInst :: Split => { Inst :: Split (InstSplit { goto1 : goto1 , goto2 : goto2 }) } _ => unreachable ! ("must be called on Split instruction, \
                               instead it was called on: {:?}" , self) , } ; * self = MaybeInst :: Compiled (filled) ; } fn half_fill_split_goto1 (& mut self , goto1 : InstPtr) { let half_filled = match * self { MaybeInst :: Split => goto1 , _ => unreachable ! ("must be called on Split instruction, \
                               instead it was called on: {:?}" , self) , } ; * self = MaybeInst :: Split1 (half_filled) ; } fn half_fill_split_goto2 (& mut self , goto2 : InstPtr) { let half_filled = match * self { MaybeInst :: Split => goto2 , _ => unreachable ! ("must be called on Split instruction, \
                               instead it was called on: {:?}" , self) , } ; * self = MaybeInst :: Split2 (half_filled) ; } fn unwrap (self) -> Inst { match self { MaybeInst :: Compiled (inst) => inst , _ => unreachable ! ("must be called on a compiled instruction, \
                               instead it was called on: {:?}" , self) , } } }
};
}
