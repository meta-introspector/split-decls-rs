// Generated macro for expand_self (function)
macro_rules! Depcrate_syn_utilsexpand_self {
() => {
// Module: crate::syn_utils
// Provides: {"expand_self"}
// Dependencies: {}
pub fn expand_self < T : VisitableMut + Clone > (input : & T , to : & Type) -> T { struct ExpandSelfVisitor < 'a > { to : & 'a Type , } impl VisitMut for ExpandSelfVisitor < '_ > { fn visit_type_mut (& mut self , i : & mut Type) { let tself : Type = parse_quote ! (Self) ; if i == & tself { * i = self . to . clone () ; } else { visit_type_mut (self , i) ; } } } let mut input = input . clone () ; input . visit_mut (& mut ExpandSelfVisitor { to }) ; input }
};
}
