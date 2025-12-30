// Generated macro for delegate (macro)
macro_rules! Depcrate_tagsdelegate {
() => {
// Module: crate::tags
// Provides: {"delegate"}
// Dependencies: {}
macro_rules ! delegate { ($ name : ident , $ type : ty) => { fn $ name < E : serde :: de :: Error > (self , v : $ type) -> Result < Self :: Value , E > { T :: deserialize (v . into_deserializer ()) . map (untagged) } } ; }
};
}
