// Generated macro for State (enum)
macro_rules! Depcrate_nfaState {
() => {
// Module: crate::nfa
// Provides: {"State"}
// Dependencies: {}
# [derive (Clone , Eq , PartialEq)] pub (crate) enum State { Char { target : StateID , ch : char } , Ranges { target : StateID , ranges : Vec < (char , char) > } , Splits { targets : Vec < StateID > , reverse : bool } , Goto { target : StateID , look : Option < hir :: Look > } , Capture { target : StateID , slot : u32 } , Fail , Match , }
};
}
