// Generated macro for link (macro)
macro_rules! Depcrate_linklink {
() => {
// Module: crate::link
// Provides: {"link"}
// Dependencies: {}
# [cfg (not (feature = "runtime"))] macro_rules ! link { ($ ($ (# [doc =$ doc : expr] # [cfg ($ cfg : meta)]) * pub fn $ name : ident ($ ($ pname : ident : $ pty : ty) , *) $ (-> $ ret : ty) *;) +) => (extern { $ ($ (# [doc =$ doc] # [cfg ($ cfg)]) * pub fn $ name ($ ($ pname : $ pty) , *) $ (-> $ ret) *;) + } $ ($ (# [doc =$ doc] # [cfg ($ cfg)]) * pub mod $ name { pub fn is_loaded () -> bool { true } }) +) }
};
}
