// Generated macro for impl_589 (impl)
macro_rules! Depcrate_dynamic_fieldimpl_589 {
() => {
// Module: crate::dynamic::field
// Provides: {"impl_589"}
// Dependencies: {}
impl Debug for FieldValue < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self . 0 { FieldValueInner :: Value (v) => write ! (f , "{}" , v) , FieldValueInner :: BorrowedAny (ty , _) | FieldValueInner :: OwnedAny (ty , _) | FieldValueInner :: WithType { ty , .. } => write ! (f , "{}" , ty) , FieldValueInner :: List (list) => match list . first () { Some (v) => { write ! (f , "[{:?}, ...]" , v) } None => { write ! (f , "[()]") } } , } } }
};
}
