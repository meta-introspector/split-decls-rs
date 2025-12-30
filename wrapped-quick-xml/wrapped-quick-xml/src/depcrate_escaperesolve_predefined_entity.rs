// Generated macro for resolve_predefined_entity (function)
macro_rules! Depcrate_escaperesolve_predefined_entity {
() => {
// Module: crate::escape
// Provides: {"resolve_predefined_entity"}
// Dependencies: {}
# [doc = " Resolves predefined XML entities or all HTML5 entities depending on the feature"] # [doc = " [`escape-html`](https://docs.rs/quick-xml/latest/quick_xml/#escape-html)."] # [doc = ""] # [doc = " Behaves like [`resolve_xml_entity`] if feature is not enabled and as"] # [doc = " [`resolve_html5_entity`] if enabled."] # [inline] pub const fn resolve_predefined_entity (entity : & str) -> Option < & 'static str > { # [cfg (not (feature = "escape-html"))] { resolve_xml_entity (entity) } # [cfg (feature = "escape-html")] { resolve_html5_entity (entity) } }
};
}
