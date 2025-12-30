// Generated macro for InlinedState (struct)
macro_rules! Depcrate_functionInlinedState {
() => {
// Module: crate::function
// Provides: {"InlinedState"}
// Dependencies: {}
struct InlinedState < 'a , R : gimli :: Reader > { entries : gimli :: EntriesRaw < 'a , 'a , R > , functions : Vec < InlinedFunction < R > > , addresses : Vec < InlinedFunctionAddress > , file : DebugFile , unit : gimli :: UnitRef < 'a , R > , ctx : & 'a Context < R > , }
};
}
