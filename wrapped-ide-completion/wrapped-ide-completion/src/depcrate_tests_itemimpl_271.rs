// Generated macro for impl_271 (impl)
macro_rules! Depcrate_tests_itemimpl_271 {
() => {
// Module: crate::tests::item
// Provides: {"impl_271"}
// Dependencies: {}
impl fmt :: Debug for CompletionItem { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("CompletionItem") ; s . field ("label" , & self . label . primary) . field ("detail_left" , & self . label . detail_left) . field ("detail_right" , & self . label . detail_right) . field ("source_range" , & self . source_range) ; if self . text_edit . len () == 1 { let atom = self . text_edit . iter () . next () . unwrap () ; s . field ("delete" , & atom . delete) ; s . field ("insert" , & atom . insert) ; } else { s . field ("text_edit" , & self . text_edit) ; } s . field ("kind" , & self . kind) ; if self . lookup () != self . label . primary { s . field ("lookup" , & self . lookup ()) ; } if let Some (detail) = & self . detail { s . field ("detail" , & detail) ; } if let Some (documentation) = & self . documentation { s . field ("documentation" , & documentation) ; } if self . deprecated { s . field ("deprecated" , & true) ; } if self . relevance != CompletionRelevance :: default () { s . field ("relevance" , & self . relevance) ; } if let Some ((ref_mode , offset)) = self . ref_match { let prefix = match ref_mode { CompletionItemRefMode :: Reference (mutability) => match mutability { Mutability :: Shared => "&" , Mutability :: Mut => "&mut " , } , CompletionItemRefMode :: Dereference => "*" , } ; s . field ("ref_match" , & format ! ("{prefix}@{offset:?}")) ; } if self . trigger_call_info { s . field ("trigger_call_info" , & true) ; } s . finish () } }
};
}
