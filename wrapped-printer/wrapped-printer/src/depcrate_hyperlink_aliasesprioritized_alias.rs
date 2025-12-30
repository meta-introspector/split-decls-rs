// Generated macro for prioritized_alias (function)
macro_rules! Depcrate_hyperlink_aliasesprioritized_alias {
() => {
// Module: crate::hyperlink::aliases
// Provides: {"prioritized_alias"}
// Dependencies: {}
# [doc = " Creates a [`HyperlinkAlias`] with a display priority."] const fn prioritized_alias (priority : i16 , name : & 'static str , description : & 'static str , format : & 'static str ,) -> HyperlinkAlias { HyperlinkAlias { name , description , format , display_priority : Some (priority) , } }
};
}
