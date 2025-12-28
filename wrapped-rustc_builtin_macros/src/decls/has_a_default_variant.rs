macro_rules! has_a_default_variant {
    () => {
        fn has_a_default_variant (item : & Annotatable) -> bool { struct HasDefaultAttrOnVariant ; impl < 'ast > rustc_ast :: visit :: Visitor < 'ast > for HasDefaultAttrOnVariant { type Result = ControlFlow < () > ; fn visit_variant (& mut self , v : & 'ast rustc_ast :: Variant) -> ControlFlow < () > { if v . attrs . iter () . any (| attr | attr . has_name (kw :: Default)) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } } item . visit_with (& mut HasDefaultAttrOnVariant) . is_break () }
    };
}

has_a_default_variant!();