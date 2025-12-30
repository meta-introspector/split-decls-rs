// Generated macro for __prefs_convert (macro)
macro_rules! Depcrate_preferences__prefs_convert {
() => {
// Module: crate::preferences
// Provides: {"__prefs_convert"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! __prefs_convert { ($ name1 : ident , $ name2 : ident) => { impl From <&$ name1 > for $ name2 { fn from (other : &$ name1) -> Self { let mut result = Self :: default () ; result . locale_preferences = other . locale_preferences ; result } } } ; ($ name1 : ident , $ name2 : ident , { $ ($ key : ident) ,* }) => { impl From <&$ name1 > for $ name2 { fn from (other : &$ name1) -> Self { let mut result = Self :: default () ; result . locale_preferences = other . locale_preferences ; $ (result .$ key = other .$ key ;) * result } } } ; }
};
}
