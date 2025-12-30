// Generated macro for impl_197 (impl)
macro_rules! Depcrate_constsimpl_197 {
() => {
// Module: crate::consts
// Provides: {"impl_197"}
// Dependencies: {}
impl PartialEq for Constant { fn eq (& self , other : & Self) -> bool { match (self , other) { (Self :: Str (ls) , Self :: Str (rs)) => ls == rs , (Self :: Binary (l) , Self :: Binary (r)) => l == r , (& Self :: Char (l) , & Self :: Char (r)) => l == r , (& Self :: Int (l) , & Self :: Int (r)) => l == r , (& Self :: F64 (l) , & Self :: F64 (r)) => { l . to_bits () == r . to_bits () && ! l . is_nan () } , (& Self :: F32 (l) , & Self :: F32 (r)) => { l . to_bits () == r . to_bits () && ! l . is_nan () } , (& Self :: Bool (l) , & Self :: Bool (r)) => l == r , (& Self :: Vec (ref l) , & Self :: Vec (ref r)) | (& Self :: Tuple (ref l) , & Self :: Tuple (ref r)) => l == r , (Self :: Repeat (lv , ls) , Self :: Repeat (rv , rs)) => ls == rs && lv == rv , (Self :: Ref (lb) , Self :: Ref (rb)) => * lb == * rb , _ => false , } } }
};
}
