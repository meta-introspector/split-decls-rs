// Generated macro for UnexposedAttr (enum)
macro_rules! Depcrate_unexposed_attrUnexposedAttr {
() => {
// Module: crate::unexposed_attr
// Provides: {"UnexposedAttr"}
// Dependencies: {}
# [doc = " Parts of `EntityKind::UnexposedAttr` that we can easily parse."] # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub enum UnexposedAttr { Enum , Options , ClosedEnum , ErrorEnum , TypedEnum , TypedExtensibleEnum , BridgedTypedef , BridgedImplicit , Bridged (String) , BridgedMutable (String) , BridgedRelated , # [doc = " `ns_returns_retained` / `cf_returns_retained` / `os_returns_retained`"] ReturnsRetained , # [doc = " `ns_returns_not_retained` / `cf_returns_not_retained` / `os_returns_not_retained`"] ReturnsNotRetained , Sendable , NonSendable , UIActor , NonIsolated , NoEscape , NoThrow , FullyUnavailable , }
};
}
