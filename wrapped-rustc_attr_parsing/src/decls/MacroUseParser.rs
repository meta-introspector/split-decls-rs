macro_rules! MacroUseParser {
    () => {
        # [doc = " `#[macro_use]` attributes can either:"] # [doc = " - Use all macros from a crate, if provided without arguments"] # [doc = " - Use specific macros from a crate, if provided with arguments `#[macro_use(macro1, macro2)]`"] # [doc = " A warning should be provided if an use all is combined with specific uses, or if multiple use-alls are used."] # [derive (Default)] pub (crate) struct MacroUseParser { state : MacroUseArgs , # [doc = " Spans of all `#[macro_use]` arguments with arguments, used for linting"] uses_attr_spans : ThinVec < Span > , # [doc = " If `state` is `UseSpecific`, stores the span of the first `#[macro_use]` argument, used as the span for this attribute"] # [doc = " If `state` is `UseAll`, stores the span of the first `#[macro_use]` arguments without arguments"] first_span : Option < Span > , }
    };
}

MacroUseParser!();