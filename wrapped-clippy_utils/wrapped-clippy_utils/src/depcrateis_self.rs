// Generated macro for is_self (function)
macro_rules! Depcrateis_self {
() => {
// Module: crate
// Provides: {"is_self"}
// Dependencies: {}
pub fn is_self (slf : & Param < '_ >) -> bool { if let PatKind :: Binding (.. , name , _) = slf . pat . kind { name . name == kw :: SelfLower } else { false } }
};
}
