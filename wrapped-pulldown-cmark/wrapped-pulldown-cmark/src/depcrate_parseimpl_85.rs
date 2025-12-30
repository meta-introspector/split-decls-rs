// Generated macro for impl_85 (impl)
macro_rules! Depcrate_parseimpl_85 {
() => {
// Module: crate::parse
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'input , F > core :: fmt :: Debug for Parser < 'input , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("Parser") . field ("text" , & self . inner . text) . field ("options" , & self . inner . options) . field ("broken_link_callback" , & self . broken_link_callback . as_ref () . map (| _ | ..) ,) . finish () } }
};
}
