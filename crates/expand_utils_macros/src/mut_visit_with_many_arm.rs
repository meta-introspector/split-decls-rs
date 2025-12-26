/// Helper macro for generating implementations for `many` fragments.
macro_rules! __generate_mut_visit_with_many_arm {
    (
        $Kind:ident, // e.g., Stmts (the ExpandedX type name)
        $AstTy:ty,   // e.g., SmallVec<ast::Stmt, 1> (the inner type)
        $kind_name:expr, // e.g., "statement"
        $flat_map_ast_elt:ident, // e.g., flat_map_stmt (method on MutVisitor)
        $visit_ast_elt:ident, // e.g., visit_stmt (method on MutVisitor)
        $args:tt, // Now captures $args as a single token tree
        $ast_to_string_elt:path, // e.g., pprust::stmt_to_string
        $make_ast:ident // e.g., make_stmts
    ) => {
        impl $crate::InvocationCollectorNode for $Kind {
            type FlatMapOutputTy = Self; // Self is SmallVec<ExpandedStmt, 1>
            type VisitOutputTy = Self;   // Self is SmallVec<ExpandedStmt, 1>

            const KIND: $crate::AstFragmentKind = $crate::AstFragmentKind::$Kind;

            fn to_annotatable(self) -> $crate::Annotatable {
                // Assuming Annotatable has a variant for each $Kind
                $crate::Annotatable::$Kind(self.0)
            }

            fn fragment_to_visit_output(fragment: $crate::AstFragment) -> Self {
                match fragment {
                    $crate::AstFragment::$Kind(nodes) => Self(nodes),
                    _ => Default::default(), // Return empty if fragment is not of this kind, or panic.
                }
            }

            fn fragment_to_flat_map_output(fragment: $crate::AstFragment) -> Self {
                match fragment {
                    $crate::AstFragment::$Kind(nodes) => Self(nodes),
                    _ => panic!("unexpected fragment kind"),
                }
            }

            fn descr() -> &'static str {
                $kind_name
            }

            fn walk<D: $crate::MutVisitor>(mut self, visitor: &mut D) -> Self {
                // Call the appropriate mut visitor method on each element
                for node in self.0.iter_mut() {
                    // $visit_ast_elt is visit_stmt, which takes &mut Stmt
                    visitor.$visit_ast_elt(node $args); // $args is now a single tt, so no need for $()*
                }
                self
            }

            fn walk_flat_map<D: $crate::MutVisitor>(self, visitor: &mut D) -> Self {
                // Call the appropriate flat_map method on the collection
                // $flat_map_ast_elt is flat_map_stmt, which takes Stmt and returns SmallVec<Stmt, 1>
                // So, we pass the inner collection (self.0) to the visitor, and wrap the result.
                Self(visitor.$flat_map_ast_elt(self.0))
            }

            fn is_mac_call(&self) -> bool {
                // This is generic and difficult for collections. Assume false for now.
                // Could iterate and check each element if it's a mac_call.
                false
            }

            fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, $crate::AddSemicolon) {
                unreachable!() // Not applicable to collections of arbitrary nodes
            }

            fn delegation(&self) -> Option<(&ast::DelegationMac, &ast::Item<Self::ItemKind>)> {
                None
            }

            fn delegation_item_kind(_deleg: Box<ast::Delegation>) -> Self::ItemKind {
                unreachable!()
            }

            fn from_item(_item: ast::Item<Self::ItemKind>) -> Self {
                unreachable!()
            }

            fn flatten_outputs(_outputs: impl Iterator<Item = Self>) -> Self {
                let mut result = Default::default(); // Assumes default impl for Self (e.g. SmallVec)
                for output in _outputs {
                    result.0.extend(output.0);
                }
                result
            }

            fn pre_flat_map_node_collect_attr<F>(_cfg: &F, _attr: &ast::Attribute) {}

            fn post_flat_map_node_collect_bang(_output: &mut Self, _add_semicolon: $crate::AddSemicolon) {}

            fn wrap_flat_map_node_walk_flat_map<D>(
                node: Self,
                _collector: &mut D,
                walk_flat_map: impl FnOnce(Self, &mut D) -> Self,
            ) -> Result<Self, Self> {
                Ok(walk_flat_map(node, _collector))
            }

            fn expand_cfg_false<D>(
                &mut self,
                _collector: &mut D,
                _pos: usize,
                _span: Span,
            ) {
            }

            fn declared_idents(&self) -> Vec<$crate::Ident> {
                vec![]
            }
        }
    };
}