// Generated macro for impl_29 (impl)
macro_rules! Depcrate_astimpl_29 {
() => {
// Module: crate::ast
// Provides: {"impl_29"}
// Dependencies: {}
impl AutoLogDemangle { # [cfg (feature = "logging")] fn new < P , W > (production : & P , ctx : & DemangleContext < W > , scope : Option < ArgScopeStack > , is_inner : bool ,) -> AutoLogDemangle where P : ? Sized + fmt :: Debug , W : DemangleWrite , { LOG_DEPTH . with (| depth | { if * depth . borrow () == 0 { println ! () ; } let indent : String = (0 .. * depth . borrow () * 4) . map (| _ | ' ') . collect () ; log ! ("{}(" , indent) ; log ! ("{}  {}{:?}" , indent , if is_inner { "as_inner: " } else { "" } , production) ; log ! ("{}  inner = {:?}" , indent , ctx . inner) ; log ! ("{}  scope = {:?}" , indent , scope) ; * depth . borrow_mut () += 1 ; }) ; AutoLogDemangle } # [cfg (not (feature = "logging"))] # [inline (always)] fn new < P , W > (_ : & P , _ : & DemangleContext < W > , _ : Option < ArgScopeStack > , _ : bool ,) -> AutoLogDemangle where P : ? Sized + fmt :: Debug , W : DemangleWrite , { AutoLogDemangle } }
};
}
