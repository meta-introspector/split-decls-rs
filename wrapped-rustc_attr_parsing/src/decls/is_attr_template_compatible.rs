macro_rules! is_attr_template_compatible {
    () => {
        # [doc = " Checks that the given meta-item is compatible with this `AttributeTemplate`."] fn is_attr_template_compatible (template : & AttributeTemplate , meta : & ast :: MetaItemKind) -> bool { let is_one_allowed_subword = | items : & [MetaItemInner] | match items { [item] => item . is_word () && template . one_of . iter () . any (| & word | item . has_name (word)) , _ => false , } ; match meta { MetaItemKind :: Word => template . word , MetaItemKind :: List (items) => template . list . is_some () || is_one_allowed_subword (items) , MetaItemKind :: NameValue (lit) if lit . kind . is_str () => template . name_value_str . is_some () , MetaItemKind :: NameValue (..) => false , } }
    };
}

is_attr_template_compatible!()