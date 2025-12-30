// Generated macro for impl_76 (impl)
macro_rules! Depcrate_errorimpl_76 {
() => {
// Module: crate::error
// Provides: {"impl_76"}
// Dependencies: {}
impl < T , E > ParseResult < T , E > { # [inline] pub fn is_ok (& self) -> bool { match * self { CommitOk (_) | PeekOk (_) => true , CommitErr (_) | PeekErr (_) => false , } } # [inline] pub fn is_err (& self) -> bool { ! self . is_ok () } pub fn as_ref (& self) -> ParseResult < & T , & E > { match * self { CommitOk (ref t) => CommitOk (t) , PeekOk (ref t) => PeekOk (t) , CommitErr (ref e) => CommitErr (e) , PeekErr (ref e) => PeekErr (Tracked { error : & e . error , offset : e . offset , }) , } } pub fn and_then < F , T2 > (self , f : F) -> F :: Output where F : FnOnce (T) -> ParseResult < T2 , E > , { match self { CommitOk (t) => match f (t) { CommitOk (t2) | PeekOk (t2) => CommitOk (t2) , PeekErr (e) => CommitErr (e . error) , CommitErr (e) => CommitErr (e) , } , PeekOk (t) => f (t) , CommitErr (e) => CommitErr (e) , PeekErr (e) => PeekErr (e) , } } pub fn map_err < F , E2 > (self , f : F) -> ParseResult < T , F :: Output > where F : FnOnce (E) -> E2 , { match self { CommitOk (t) => CommitOk (t) , PeekOk (t) => PeekOk (t) , CommitErr (e) => CommitErr (f (e)) , PeekErr (e) => PeekErr (Tracked { error : f (e . error) , offset : e . offset , }) , } } pub fn map < F , T2 > (self , f : F) -> ParseResult < F :: Output , E > where F : FnOnce (T) -> T2 , { match self { CommitOk (t) => CommitOk (f (t)) , PeekOk (t) => PeekOk (f (t)) , CommitErr (e) => CommitErr (e) , PeekErr (e) => PeekErr (e) , } } }
};
}
