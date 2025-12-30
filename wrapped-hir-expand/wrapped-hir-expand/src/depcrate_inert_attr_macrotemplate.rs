// Generated macro for template (macro)
macro_rules! Depcrate_inert_attr_macrotemplate {
() => {
// Module: crate::inert_attr_macro
// Provides: {"template"}
// Dependencies: {}
# [doc = " A convenience macro for constructing attribute templates."] # [doc = " E.g., `template!(Word, List: \"description\")` means that the attribute"] # [doc = " supports forms `#[attr]` and `#[attr(description)]`."] macro_rules ! template { (Word) => { template ! (@ true , None , None) } ; (List : $ descr : expr) => { template ! (@ false , Some ($ descr) , None) } ; (NameValueStr : $ descr : expr) => { template ! (@ false , None , Some ($ descr)) } ; (Word , List : $ descr : expr) => { template ! (@ true , Some ($ descr) , None) } ; (Word , NameValueStr : $ descr : expr) => { template ! (@ true , None , Some ($ descr)) } ; (List : $ descr1 : expr , NameValueStr : $ descr2 : expr) => { template ! (@ false , Some ($ descr1) , Some ($ descr2)) } ; (Word , List : $ descr1 : expr , NameValueStr : $ descr2 : expr) => { template ! (@ true , Some ($ descr1) , Some ($ descr2)) } ; (@ $ word : expr , $ list : expr , $ name_value_str : expr) => { AttributeTemplate { word : $ word , list : $ list , name_value_str : $ name_value_str } } ; }
};
}
