// Generated macro for parse_direct_protocols (function)
macro_rules! Depcrate_protocolparse_direct_protocols {
() => {
// Module: crate::protocol
// Provides: {"parse_direct_protocols"}
// Dependencies: {}
# [doc = " Parse the directly referenced protocols of a declaration."] pub (crate) fn parse_direct_protocols < 'clang > (entity : & Entity < 'clang > , _context : & Context < '_ > ,) -> Vec < Entity < 'clang > > { let mut protocols = Vec :: new () ; # [allow (clippy :: single_match)] immediate_children (entity , | child , _span | match child . get_kind () { EntityKind :: ObjCProtocolRef => { let child = child . get_reference () . expect ("ObjCProtocolRef to reference entity") ; if child == * entity { error ! (? entity , "recursive protocol") ; } else { protocols . push (child) ; } } _ => { } }) ; protocols }
};
}
