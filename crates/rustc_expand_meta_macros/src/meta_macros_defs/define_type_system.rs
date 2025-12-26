#[macro_export]
macro_rules! define_type_system {
    () => {
        // CollectorSpecial! abstracts the InvocationCollector type.
        // It takes type parameters which are then passed to the underlying InvocationCollector.
        include!("meta_macros_defs/collector_special.rs");

        // CtxSpecial! abstracts the RefAMutExpandContext_B_DRT type.
        // It takes type parameters which are then passed to the underlying RefAMutExpandContext_B_DRT.
        include!("meta_macros_defs/ctx_special.rs");

        // ExpanderStruct! generates the MacroExpander struct definition.
        // It uses CtxSpecial! to define the type of the `cx` field,
        // thereby abstracting away the explicit generics on the struct itself.
        include!("meta_macros_defs/expander_struct.rs");

        // This macro wraps the contents of the impl MacroExpander block,
        // implicitly adding the necessary generics and trait bounds.
        include!("meta_macros_defs/impl_for_macro_expander.rs");

        // MacroExpanderABDRT! expands to the generic MacroExpander struct with type parameters.
        include!("meta_macros_defs/macro_expander_abdr_t.rs");

        // REFAMutExtCtxt! expands to a mutable reference to ExtCtxt with type parameters.
        include!("meta_macros_defs/ref_a_mut_ext_ctxt.rs");

        // MacroExpanderNew! creates a new MacroExpander instance.
        include!("meta_macros_defs/macro_expander_new.rs");

        // --- Macros for InvocationCollector.rs ---

        // Generates the InvocationCollector struct definition.
        include!("meta_macros_defs/make_invocation_collector_struct.rs");

        // Generates the InvocationCollectorCfgExt trait definition.
        include!("meta_macros_defs/make_invocation_collector_cfg_ext_trait.rs");

        // Generates the InvocationCollectorCfgExt impl block.
        include!("meta_macros_defs/make_invocation_collector_cfg_ext_impl.rs");

        // Generates the InvocationMutVisitor impl block for InvocationCollector.
        include!("meta_macros_defs/make_invocation_mut_visitor_impl.rs");

        // Generates the MutVisitor impl block for InvocationCollector.
        include!("meta_macros_defs/make_mut_visitor_impl.rs");
        include!("meta_macros_defs/define_impl_with_drt.rs");

        // Type alias for InvocationCollector for brevity and consistency
        macro_rules! InvocationContextABDRT {
            () => { InvocationCollector<'a, 'b, DRT> };
        }

        // --- Macros for dummy_visitor_support.rs ---

        // Generates the DummyVisitor struct definition.
        macro_rules! make_dummy_visitor_struct {
            ($($body:tt)*) => {
                pub struct DummyVisitor<'a, 'b, DRT: OpaqueDeriveResolution + 'static> {
                    $($body)*
                }
            };
        }

        // Generates the MutVisitor impl block for DummyVisitor.
        macro_rules! make_dummy_visitor_mut_visitor_impl {
            ($struct_name:ident, $($body:tt)*) => {
                impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> MutVisitor for $struct_name<'a, 'b, DRT> {
                    $($body)*
                }
            };
        }

        // Generates the HasNodeId impl block for DummyVisitor.
        macro_rules! make_dummy_visitor_has_node_id_impl {
            ($struct_name:ident, $($body:tt)*) => {
                impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> HasNodeId for $struct_name<'a, 'b, DRT> {
                    $($body)*
                }
            };
        }

        // Generates the HasAttrs impl block for DummyVisitor.
        macro_rules! make_dummy_visitor_has_attrs_impl {
            ($struct_name:ident, $($body:tt)*) => {
                impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> HasAttrs for $struct_name<'a, 'b, DRT> {
                    $($body)*
                }
            };
        }
    };
}