// Generated macro for AutoDemangleContextInnerBarrier (struct)
macro_rules! Depcrate_astAutoDemangleContextInnerBarrier {
() => {
// Module: crate::ast
// Provides: {"AutoDemangleContextInnerBarrier"}
// Dependencies: {}
# [doc (hidden)] # [derive (Debug)] pub struct AutoDemangleContextInnerBarrier < 'ctx , 'a , W > where W : 'a + DemangleWrite , 'a : 'ctx , { ctx : & 'ctx mut DemangleContext < 'a , W > , saved_inner : Vec < & 'a dyn DemangleAsInner < 'a , W > > , }
};
}
