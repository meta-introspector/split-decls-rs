// Generated macro for impl_28 (impl)
macro_rules! Depcrate_navigation_targetimpl_28 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_28"}
// Dependencies: {}
impl fmt :: Debug for NavigationTarget { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_struct ("NavigationTarget") ; macro_rules ! opt { ($ ($ name : ident) *) => { $ (if let Some (it) = & self .$ name { f . field (stringify ! ($ name) , it) ; }) * } } f . field ("file_id" , & self . file_id) . field ("full_range" , & self . full_range) ; opt ! (focus_range) ; f . field ("name" , & self . name) ; opt ! (kind container_name description docs) ; f . finish () } }
};
}
