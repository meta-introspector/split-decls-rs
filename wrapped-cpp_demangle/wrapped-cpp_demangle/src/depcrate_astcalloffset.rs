// Generated macro for CallOffset (enum)
macro_rules! Depcrate_astCallOffset {
() => {
// Module: crate::ast
// Provides: {"CallOffset"}
// Dependencies: {}
# [doc = " The `<call-offset>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <call-offset> ::= h <nv-offset> _"] # [doc = "               ::= v <v-offset> _"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum CallOffset { # [doc = " A non-virtual offset."] NonVirtual (NvOffset) , # [doc = " A virtual offset."] Virtual (VOffset) , }
};
}
