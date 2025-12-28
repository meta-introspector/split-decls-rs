macro_rules! deps {
    () => {
        AdjustmentHints!();
        LifetimeElisionHints!();
        DiscriminantHints!();
        InlayFieldsToResolve!();
        ClosureReturnTypeHints!();
        AdjustmentHintsMode!();
        GenericParameterHints!();
    };
}

macro_rules! InlayHintsConfig {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct InlayHintsConfig < 'a > { pub render_colons : bool , pub type_hints : bool , pub sized_bound : bool , pub discriminant_hints : DiscriminantHints , pub parameter_hints : bool , pub generic_parameter_hints : GenericParameterHints , pub chaining_hints : bool , pub adjustment_hints : AdjustmentHints , pub adjustment_hints_disable_reborrows : bool , pub adjustment_hints_mode : AdjustmentHintsMode , pub adjustment_hints_hide_outside_unsafe : bool , pub closure_return_type_hints : ClosureReturnTypeHints , pub closure_capture_hints : bool , pub binding_mode_hints : bool , pub implicit_drop_hints : bool , pub implied_dyn_trait_hints : bool , pub lifetime_elision_hints : LifetimeElisionHints , pub param_names_for_lifetime_elision_hints : bool , pub hide_named_constructor_hints : bool , pub hide_closure_initialization_hints : bool , pub hide_closure_parameter_hints : bool , pub range_exclusive_hints : bool , pub closure_style : ClosureStyle , pub max_length : Option < usize > , pub closing_brace_hints_min_lines : Option < usize > , pub fields_to_resolve : InlayFieldsToResolve , pub minicore : MiniCore < 'a > , }
    };
}

InlayHintsConfig!()