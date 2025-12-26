#[macro_export]
macro_rules! define_syntax_extension_types {
    ($DRT:ident, $MacroRulesMacroExpanderDRT:ident) => {
        pub struct SyntaxExtension<DRT: OpaqueDeriveResolution + 'static> {
            pub kind: SyntaxExtensionKind<DRT>,
            pub allow_internal_unstable: bool,
            pub allow_internal_unsafe: bool,
            pub local_inner_macros: bool,
            pub edition: Edition,
            pub ident: Option<Ident>,
            pub attrs: ThinVec<Attribute>,
        }

        impl<DRT: OpaqueDeriveResolution + 'static> SyntaxExtensionTrait<DRT> for SyntaxExtension<DRT> {
            fn new_macro_extension(
                sess: &Session,
                kind: Arc<dyn MacroKindTrait<DRT>>,
                span: Span,
                allow_internal_attrs: Vec<Attribute>,
                edition: Edition,
                name: Symbol,
                attrs: &[Attribute],
                is_local: bool,
            ) -> Self {
                // This is a continuation point. The `kind` (Arc<dyn MacroKindTrait>) needs to be
                // mapped back to a concrete SyntaxExtensionKind. For now, we'll use a placeholder.
                // A proper implementation would involve a downcast or a more structured enum.
                SyntaxExtension::new(
                    sess,
                    kind, // Pass the Arc<dyn MacroKindTrait<DRT>> directly
                    span,
                    allow_internal_attrs,
                    edition,
                    name,
                    attrs,
                    is_local,
                )
            }

            fn dummy_bang_extension(edition: Edition) -> Self {
                Self::dummy_bang(edition)
            }

            fn dummy_derive_extension(edition: Edition) -> Self {
                Self::dummy_derive(edition)
            }

            fn non_macro_attr_extension(edition: Edition) -> Self {
                Self::non_macro_attr(edition)
            }

            fn glob_delegation_extension(def_id: DefId, impl_def_id: Option<DefId>, edition: Edition) -> Self {
                Self::glob_delegation(def_id, impl_def_id, edition)
            }

            fn get_macro_kind(&self) -> &dyn MacroKindTrait<DRT> {
                &*self.kind
            }

            fn as_bang(&self) -> Option<&dyn Any> {
                match &self.kind {
                    SyntaxExtensionKind::Bang(expander) => Some(&**expander as &dyn Any),
                    _ => None,
                }
            }
            fn as_attr(&self) -> Option<&dyn Any> {
                match &self.kind {
                    SyntaxExtensionKind::Attr(expander) => Some(&**expander as &dyn Any),
                    _ => None,
                }
            }
            fn as_legacy_bang(&self) -> Option<&dyn Any> {
                match &self.kind {
                    SyntaxExtensionKind::LegacyBang(expander) => Some(&**expander as &dyn Any),
                    _ => None,
                }
            }
            fn as_derive(&self) -> Option<&dyn Any> {
                match &self.kind {
                    SyntaxExtensionKind::Derive(expander) => Some(&**expander as &dyn Any),
                    _ => None,
                }
            }
            fn as_legacy_derive(&self) -> Option<&dyn Any> {
                match &self.kind {
                    SyntaxExtensionKind::LegacyDerive(expander) => Some(&**expander as &dyn Any),
                    _ => None,
                }
            }
            fn as_glob_delegation(&self) -> Option<&dyn Any> {
                match &self.kind {
                    SyntaxExtensionKind::GlobDelegation(expander) => Some(&**expander as &dyn Any),
                    _ => None,
                }
            }
        }


        impl<DRT: OpaqueDeriveResolution + 'static> SyntaxExtension<DRT> {
            pub fn new(
                _sess: &Session,
                kind: SyntaxExtensionKind<DRT>,
                _span: Span,
                _allow_internal_attrs: Vec<Attribute>, // This argument is Vec::new() in macro_rules.rs
                edition: Edition,
                name: Symbol,
                attrs: &[Attribute],
                _is_local: bool,
            ) -> Self {
                SyntaxExtension {
                    kind,
                    allow_internal_unstable: false, // Default for now
                    allow_internal_unsafe: false,   // Default for now
                    local_inner_macros: false,      // Default for now
                    edition,
                    ident: Some(Ident::with_dummy_span(name)),
                    attrs: ThinVec::from_slice(attrs),
                }
            }

            pub fn dummy_bang(edition: Edition) -> Self {
                SyntaxExtension {
                    kind: SyntaxExtensionKind::Bang(Arc::new(DummyBang(ErrorGuaranteed::dummy()))),
                    allow_internal_unstable: false,
                    allow_internal_unsafe: false,
                    local_inner_macros: false,
                    edition,
                    ident: None,
                    attrs: ThinVec::new(),
                }
            }
            pub fn dummy_derive(edition: Edition) -> Self {
                SyntaxExtension {
                    kind: SyntaxExtensionKind::Derive(Arc::new(DummyMultiItemModifier(ErrorGuaranteed::dummy()))),
                    allow_internal_unstable: false,
                    allow_internal_unsafe: false,
                    local_inner_macros: false,
                    edition,
                    ident: None,
                    attrs: ThinVec::new(),
                }
            }
            pub fn non_macro_attr(edition: Edition) -> Self {
                SyntaxExtension {
                    kind: SyntaxExtensionKind::NonMacroAttr,
                    allow_internal_unstable: false,
                    allow_internal_unsafe: false,
                    local_inner_macros: false,
                    edition,
                    ident: None,
                    attrs: ThinVec::new(),
                }
            }
            pub fn glob_delegation(def_id: DefId, impl_def_id: Option<DefId>, edition: Edition) -> Self {
                SyntaxExtension {
                    kind: SyntaxExtensionKind::GlobDelegation(Arc::new(DummyGlobDelegation(ErrorGuaranteed::dummy()))),
                    allow_internal_unstable: false,
                    allow_internal_unsafe: false,
                    local_inner_macros: false,
                    edition,
                    ident: None,
                    attrs: ThinVec::new(),
                }
            }

            pub fn into_arc_trait_object(self) -> Arc<dyn SyntaxExtensionTrait<DRT>> {
                Arc::new(self)
            }
        }

        pub enum SyntaxExtensionKind<DRT: OpaqueDeriveResolution + 'static, MacroRulesMacroExpanderDRT> {
            /// `macro_rules!` items
            MacroRules(Arc<MacroRulesMacroExpanderDRT>),
            /// proc macros
            Bang(Arc<dyn BangProcMacro<DRT> + Send + Sync>),
            /// proc macro attributes
            Attr(Arc<dyn AttrProcMacro<DRT> + Send + Sync>),
            /// proc macros like `#[derive]`
            Derive(Arc<dyn MultiItemModifier<DRT> + Send + Sync>),
            /// Legacy `macro_rules!` type, can be macro or derive. This is for compatibility with older definitions.
            LegacyBang(Arc<MacroExpanderFn>),
            LegacyAttr(Arc<dyn MultiItemModifier<DRT> + Send + Sync>),
            LegacyDerive(Arc<dyn MultiItemModifier<DRT> + Send + Sync>),
            /// Dummy/inert attributes, for `#[macro_export]` etc.
            NonMacroAttr,
            /// Trait/impl glob delegation (RFC 1645)
            GlobDelegation(Arc<dyn GlobDelegationExpander<DRT> + Send + Sync>),
        }

        impl<DRT: OpaqueDeriveResolution + 'static, MacroRulesMacroExpanderDRT> MacroKindTrait for SyntaxExtensionKind<DRT, MacroRulesMacroExpanderDRT> {
            fn get_name(&self) -> String {
                match self {
                    SyntaxExtensionKind::MacroRules(_) => "macro_rules".to_string(),
                    SyntaxExtensionKind::Bang(_) => "bang".to_string(),
                    SyntaxExtensionKind::Attr(_) => "attr".to_string(),
                    SyntaxExtensionKind::Derive(_) => "derive".to_string(),
                    SyntaxExtensionKind::LegacyBang(_) => "legacy_bang".to_string(),
                    SyntaxExtensionKind::LegacyAttr(_) => "legacy_attr".to_string(),
                    SyntaxExtensionKind::LegacyDerive(_) => "legacy_derive".to_string(),
                    SyntaxExtensionKind::NonMacroAttr => "non_macro_attr".to_string(),
                    SyntaxExtensionKind::GlobDelegation(_) => "glob_delegation".to_string(),
                }
            }
        }
    };
}