// Generated macro for NfaState (enum)
macro_rules! Depcrate_readerNfaState {
() => {
// Module: crate::reader
// Provides: {"NfaState"}
// Dependencies: {}
# [doc = " An NFA state is a state that can be visited in the NFA parser."] # [doc = ""] # [doc = " Given the simplicity of the machine, a subset of NFA states double as DFA"] # [doc = " states. NFA states that only have incoming epsilon transitions are"] # [doc = " optimized out when converting the machine to a DFA."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] enum NfaState { EndFieldTerm = 200 , InRecordTerm = 201 , End = 202 , StartRecord = 0 , StartField = 1 , InField = 2 , InQuotedField = 3 , InEscapedQuote = 4 , InDoubleEscapedQuote = 5 , InComment = 6 , EndFieldDelim = 7 , EndRecord = 8 , CRLF = 9 , }
};
}
