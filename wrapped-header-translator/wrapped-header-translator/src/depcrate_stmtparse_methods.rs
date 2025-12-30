// Generated macro for parse_methods (function)
macro_rules! Depcrate_stmtparse_methods {
() => {
// Module: crate::stmt
// Provides: {"parse_methods"}
// Dependencies: {}
fn parse_methods (entity : & Entity < '_ > , get_data : impl Fn (& str) -> MethodData , thread_safety : & ThreadSafety , is_pub : bool , context : & Context < '_ > ,) -> (Vec < Method > , Vec < String >) { let mut methods = Vec :: new () ; let mut designated_initializers = Vec :: new () ; for entity in method_or_property_entities (entity , & get_data) { match entity . get_kind () { EntityKind :: ObjCInstanceMethodDecl | EntityKind :: ObjCClassMethodDecl => { let selector = entity . get_name () . expect ("method selector") ; let data = get_data (& selector) ; if let Some ((designated_initializer , method)) = Method :: parse_method (entity , data , thread_safety , is_pub , context) { if designated_initializer { designated_initializers . push (method . selector . clone ()) ; } methods . push (method) ; } } EntityKind :: ObjCPropertyDecl => { let partial = Method :: partial_property (entity) ; let getter_data = get_data (& partial . getter_sel) ; let setter_data = partial . setter_sel . as_ref () . map (| setter_sel | get_data (setter_sel)) ; let (getter , setter) = Method :: parse_property (partial , getter_data , setter_data , thread_safety , is_pub , context ,) ; if let Some (getter) = getter { methods . push (getter) ; } if let Some (setter) = setter { methods . push (setter) ; } } kind => unreachable ! ("method/property entity {kind:?}") , } } (methods , designated_initializers) }
};
}
